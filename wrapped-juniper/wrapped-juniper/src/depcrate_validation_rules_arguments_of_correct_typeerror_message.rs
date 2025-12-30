// Generated macro for error_message (function)
macro_rules! Depcrate_validation_rules_arguments_of_correct_typeerror_message {
() => {
// Module: crate::validation::rules::arguments_of_correct_type
// Provides: {"error_message"}
// Dependencies: {}
fn error_message (arg_name : impl fmt :: Display , msg : impl fmt :: Display) -> String { format ! ("Invalid value for argument \"{arg_name}\", reason: {msg}") }
};
}
