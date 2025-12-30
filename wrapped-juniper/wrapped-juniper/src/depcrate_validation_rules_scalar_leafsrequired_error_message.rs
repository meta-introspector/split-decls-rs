// Generated macro for required_error_message (function)
macro_rules! Depcrate_validation_rules_scalar_leafsrequired_error_message {
() => {
// Module: crate::validation::rules::scalar_leafs
// Provides: {"required_error_message"}
// Dependencies: {}
fn required_error_message (field_name : impl fmt :: Display , type_name : impl fmt :: Display) -> String { format ! (r#"Field "{field_name}" of type "{type_name}" must have a selection of subfields. Did you mean "{field_name} {{ ... }}"?"# ,) }
};
}
