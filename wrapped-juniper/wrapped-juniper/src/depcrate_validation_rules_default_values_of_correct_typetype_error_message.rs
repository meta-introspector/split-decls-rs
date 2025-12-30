// Generated macro for type_error_message (function)
macro_rules! Depcrate_validation_rules_default_values_of_correct_typetype_error_message {
() => {
// Module: crate::validation::rules::default_values_of_correct_type
// Provides: {"type_error_message"}
// Dependencies: {}
fn type_error_message (arg_name : impl fmt :: Display , type_name : impl fmt :: Display , reason : impl fmt :: Display ,) -> String { format ! ("Invalid default value for argument \"{arg_name}\", expected type \"{type_name}\", \
         reason: {reason}" ,) }
};
}
