// Generated macro for field_error_message (function)
macro_rules! Depcrate_validation_rules_known_argument_namesfield_error_message {
() => {
// Module: crate::validation::rules::known_argument_names
// Provides: {"field_error_message"}
// Dependencies: {}
fn field_error_message (arg_name : & str , field_name : & str , type_name : & str) -> String { format ! (r#"Unknown argument "{arg_name}" on field "{field_name}" of type "{type_name}""#) }
};
}
