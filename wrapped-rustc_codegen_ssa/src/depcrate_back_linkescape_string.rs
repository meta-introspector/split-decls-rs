// Generated macro for escape_string (function)
macro_rules! Depcrate_back_linkescape_string {
() => {
// Module: crate::back::link
// Provides: {"escape_string"}
// Dependencies: {}
fn escape_string (s : & [u8]) -> String { match str :: from_utf8 (s) { Ok (s) => s . to_owned () , Err (_) => format ! ("Non-UTF-8 output: {}" , s . escape_ascii ()) , } }
};
}
