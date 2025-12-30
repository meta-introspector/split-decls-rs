// Generated macro for unification_error (function)
macro_rules! Depcrate_validation_input_valueunification_error {
() => {
// Module: crate::validation::input_value
// Provides: {"unification_error"}
// Dependencies: {}
fn unification_error (var_name : impl Display , var_pos : & SourcePosition , path : & Path < '_ > , message : impl Display ,) -> RuleError { RuleError :: new (& format ! (r#"Variable "${var_name}" got invalid value. {path}{message}."#) , & [* var_pos] ,) }
};
}
