// Generated macro for directive_error_message (function)
macro_rules! Depcrate_validation_rules_known_argument_namesdirective_error_message {
() => {
// Module: crate::validation::rules::known_argument_names
// Provides: {"directive_error_message"}
// Dependencies: {}
fn directive_error_message (arg_name : & str , directive_name : & str) -> String { format ! (r#"Unknown argument "{arg_name}" on directive "{directive_name}""#) }
};
}
