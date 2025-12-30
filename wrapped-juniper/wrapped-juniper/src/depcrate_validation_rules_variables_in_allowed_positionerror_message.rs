// Generated macro for error_message (function)
macro_rules! Depcrate_validation_rules_variables_in_allowed_positionerror_message {
() => {
// Module: crate::validation::rules::variables_in_allowed_position
// Provides: {"error_message"}
// Dependencies: {}
fn error_message (var_name : impl fmt :: Display , type_name : impl fmt :: Display , expected_type_name : impl fmt :: Display ,) -> String { format ! ("Variable \"{var_name}\" of type \"{type_name}\" used in position expecting type \"{expected_type_name}\"" ,) }
};
}
