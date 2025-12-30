// Generated macro for error_message (function)
macro_rules! Depcrate_validation_rules_variables_are_input_typeserror_message {
() => {
// Module: crate::validation::rules::variables_are_input_types
// Provides: {"error_message"}
// Dependencies: {}
fn error_message (var_name : impl fmt :: Display , type_name : impl fmt :: Display) -> String { format ! ("Variable \"{var_name}\" cannot be of non-input type \"{type_name}\"") }
};
}
