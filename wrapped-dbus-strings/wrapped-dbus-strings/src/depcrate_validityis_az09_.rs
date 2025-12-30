// Generated macro for is_az09_ (function)
macro_rules! Depcrate_validityis_az09_ {
() => {
// Module: crate::validity
// Provides: {"is_az09_"}
// Dependencies: {}
fn is_az09_ (b : u8) -> Result < () , () > { match b { b'A' ..= b'Z' | b'a' ..= b'z' | b'0' ..= b'9' | b'_' => Ok (()) , _ => Err (()) , } }
};
}
