// Generated macro for to_string (function)
macro_rules! Depcrate_inputto_string {
() => {
// Module: crate::input
// Provides: {"to_string"}
// Dependencies: {}
# [track_caller] fn to_string (value : std :: ffi :: OsString) -> String { match value . into_string () { Ok (s) => s , Err (value) => { let err = std :: str :: from_utf8 (value . as_encoded_bytes ()) . unwrap_err () ; panic ! ("{err}") } } }
};
}
