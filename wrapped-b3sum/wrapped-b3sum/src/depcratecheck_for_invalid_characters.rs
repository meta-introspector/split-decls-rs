// Generated macro for check_for_invalid_characters (function)
macro_rules! Depcratecheck_for_invalid_characters {
() => {
// Module: crate
// Provides: {"check_for_invalid_characters"}
// Dependencies: {}
fn check_for_invalid_characters (utf8_path : & str) -> anyhow :: Result < () > { if utf8_path . contains ('\0') { bail ! ("Null character in path") ; } if utf8_path . contains ('�') { bail ! ("Unicode replacement character in path") ; } if cfg ! (windows) && utf8_path . contains ('\\') { bail ! ("Backslash in path") ; } Ok (()) }
};
}
