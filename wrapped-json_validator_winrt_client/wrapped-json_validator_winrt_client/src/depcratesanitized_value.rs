// Generated macro for sanitized_value (function)
macro_rules! Depcratesanitized_value {
() => {
// Module: crate
// Provides: {"sanitized_value"}
// Dependencies: {}
# [test] fn sanitized_value () -> Result < () > { let schema = h ! (r#"
    {
        "properties": {
            "name": {
                "type": "string"
            },
            "age": {
                "type": "integer"
            }
        }
    }
    "#) ; let value = h ! (r#"
    {
        "name": "Kenny",
        "age": 21 
    }
    "#) ; let validator = JsonValidator :: CreateInstance (schema) ? ; let sanitized = validator . Validate (value) ? ; assert_eq ! (sanitized , r#"{"age":21,"name":"Kenny"}"#) ; Ok (()) }
};
}
