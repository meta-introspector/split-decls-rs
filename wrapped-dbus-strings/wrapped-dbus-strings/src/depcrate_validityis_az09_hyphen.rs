// Generated macro for is_az09_hyphen (function)
macro_rules! Depcrate_validityis_az09_hyphen {
() => {
// Module: crate::validity
// Provides: {"is_az09_hyphen"}
// Dependencies: {}
fn is_az09_hyphen (b : u8) -> Result < () , () > { match b { b'A' ..= b'Z' | b'a' ..= b'z' | b'0' ..= b'9' | b'_' | b'-' => Ok (()) , _ => Err (()) , } }
};
}
