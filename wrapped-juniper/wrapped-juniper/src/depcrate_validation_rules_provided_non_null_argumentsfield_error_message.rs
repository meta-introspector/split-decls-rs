// Generated macro for field_error_message (function)
macro_rules! Depcrate_validation_rules_provided_non_null_argumentsfield_error_message {
() => {
// Module: crate::validation::rules::provided_non_null_arguments
// Provides: {"field_error_message"}
// Dependencies: {}
fn field_error_message (field_name : impl fmt :: Display , arg_name : impl fmt :: Display , type_name : impl fmt :: Display ,) -> String { format ! (r#"Field "{field_name}" argument "{arg_name}" of type "{type_name}" is required but not provided"# ,) }
};
}
