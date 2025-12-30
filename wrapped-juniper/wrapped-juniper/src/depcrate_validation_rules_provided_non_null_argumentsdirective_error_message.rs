// Generated macro for directive_error_message (function)
macro_rules! Depcrate_validation_rules_provided_non_null_argumentsdirective_error_message {
() => {
// Module: crate::validation::rules::provided_non_null_arguments
// Provides: {"directive_error_message"}
// Dependencies: {}
fn directive_error_message (directive_name : impl fmt :: Display , arg_name : impl fmt :: Display , type_name : impl fmt :: Display ,) -> String { format ! (r#"Directive "@{directive_name}" argument "{arg_name}" of type "{type_name}" is required but not provided"# ,) }
};
}
