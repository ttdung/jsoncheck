use serde_json::json;
use jsonschema::{Draft, JSONSchema};

fn main() {
    // Define your JSON schema
    let schema = json!({
        "type": "object",
        "properties": {
            "name": { "type": "string" },
            "age": { "type": "integer" }
        },
        "required": ["name", "age"]
    });

    // Define your JSON data
    let data = json!({
        "name": "John Doe",
        "age": 23
    });

    println!(
        "json data {}",
        data
    );
    println!(
        "json schema {}",
        schema
    );
    
    // Compile the schema
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .expect("A valid schema");

    // Validate the data against the schema
    let result = compiled_schema.validate(&data);

    // Check the validation result
    if let Err(errors) = result {
        for error in errors {
            println!("Validation error: {}", error);
            println!("Instance path: {}", error.instance_path);
        }
    } else {
        println!("JSON data is valid against the schema.");
    }
}
