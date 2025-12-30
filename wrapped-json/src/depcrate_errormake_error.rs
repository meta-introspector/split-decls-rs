// Generated macro for make_error (function)
macro_rules! Depcrate_errormake_error {
() => {
// Module: crate::error
// Provides: {"make_error"}
// Dependencies: {}
fn make_error (mut msg : String) -> Error { let (line , column) = parse_line_col (& mut msg) . unwrap_or ((0 , 0)) ; Error { err : Box :: new (ErrorImpl { code : ErrorCode :: Message (msg . into_boxed_str ()) , line , column , }) , } }
};
}
