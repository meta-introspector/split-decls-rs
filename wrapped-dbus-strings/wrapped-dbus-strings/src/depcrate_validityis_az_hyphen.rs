// Generated macro for is_az_hyphen (function)
macro_rules! Depcrate_validityis_az_hyphen {
() => {
// Module: crate::validity
// Provides: {"is_az_hyphen"}
// Dependencies: {}
fn is_az_hyphen (b : u8) -> Result < () , () > { match b { b'A' ..= b'Z' | b'a' ..= b'z' | b'_' | b'-' => Ok (()) , _ => Err (()) , } }
};
}
