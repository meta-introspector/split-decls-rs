// Generated macro for error_message (function)
macro_rules! Depcrate_validation_rules_no_undefined_variableserror_message {
() => {
// Module: crate::validation::rules::no_undefined_variables
// Provides: {"error_message"}
// Dependencies: {}
fn error_message (var_name : & str , op_name : Option < & str >) -> String { if let Some (op_name) = op_name { format ! (r#"Variable "${var_name}" is not defined by operation "{op_name}""#) } else { format ! (r#"Variable "${var_name}" is not defined"#) } }
};
}
