// Generated macro for split_nonutf8_once (function)
macro_rules! Depcratesplit_nonutf8_once {
() => {
// Module: crate
// Provides: {"split_nonutf8_once"}
// Dependencies: {}
fn split_nonutf8_once (b : & OsStr) -> (& str , Option < & OsStr >) { match b . try_str () { Ok (s) => (s , None) , Err (err) => { let (valid , after_valid) = unsafe { ext :: split_at (b , err . valid_up_to ()) } ; let valid = valid . try_str () . unwrap () ; (valid , Some (after_valid)) } } }
};
}
