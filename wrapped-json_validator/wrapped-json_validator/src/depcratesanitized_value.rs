// Generated macro for sanitized_value (function)
macro_rules! Depcratesanitized_value {
() => {
// Module: crate
// Provides: {"sanitized_value"}
// Dependencies: {}
# [test] fn sanitized_value () { unsafe { let schema = r#"
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
        "# ; let mut handle = 0 ; assert_eq ! (S_OK , CreateJsonValidator (schema . as_ptr () , schema . len () , & mut handle)) ; let value = r#"
        {
            "name": "Kenny",
            "age": 21 
        }
        "# ; let mut sanitized_alloc = std :: ptr :: null_mut () ; let mut sanitized_len = 0 ; assert_eq ! (S_OK , ValidateJson (handle , value . as_ptr () , value . len () , & mut sanitized_alloc , & mut sanitized_len)) ; let sanitized = std :: slice :: from_raw_parts (sanitized_alloc , sanitized_len) ; let sanitized = String :: from_utf8_lossy (sanitized) . into_owned () ; CoTaskMemFree (Some (sanitized_alloc as _)) ; assert_eq ! (sanitized , r#"{"age":21,"name":"Kenny"}"#) ; CloseJsonValidator (handle) ; } }
};
}
