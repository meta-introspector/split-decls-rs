// Generated macro for error_message (function)
macro_rules! Depcrate_validation_rules_fields_on_correct_typeerror_message {
() => {
// Module: crate::validation::rules::fields_on_correct_type
// Provides: {"error_message"}
// Dependencies: {}
fn error_message (field : & str , type_name : & str) -> String { format ! (r#"Unknown field "{field}" on type "{type_name}""#) }
};
}
