// Generated macro for escape_string (function)
macro_rules! Depcrate_aot_shells_fishescape_string {
() => {
// Module: crate::aot::shells::fish
// Provides: {"escape_string"}
// Dependencies: {}
fn escape_string (string : & str , escape_comma : bool) -> String { let string = string . replace ('\\' , "\\\\") . replace ('\'' , "\\'") ; if escape_comma { string . replace (',' , "\\,") } else { string } }
};
}
