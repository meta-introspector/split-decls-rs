// Generated macro for non_null_error_message (function)
macro_rules! Depcrate_validation_rules_default_values_of_correct_typenon_null_error_message {
() => {
// Module: crate::validation::rules::default_values_of_correct_type
// Provides: {"non_null_error_message"}
// Dependencies: {}
fn non_null_error_message (arg_name : impl fmt :: Display , type_name : impl fmt :: Display) -> String { format ! ("Argument \"{arg_name}\" has type \"{type_name}\" and is not nullable, \
         so it can't have a default value" ,) }
};
}
