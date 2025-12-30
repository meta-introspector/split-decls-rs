// Generated macro for no_allowed_error_message (function)
macro_rules! Depcrate_validation_rules_scalar_leafsno_allowed_error_message {
() => {
// Module: crate::validation::rules::scalar_leafs
// Provides: {"no_allowed_error_message"}
// Dependencies: {}
fn no_allowed_error_message (field_name : impl fmt :: Display , type_name : impl fmt :: Display) -> String { format ! (r#"Field "{field_name}" must not have a selection since type {type_name} has no subfields"# ,) }
};
}
