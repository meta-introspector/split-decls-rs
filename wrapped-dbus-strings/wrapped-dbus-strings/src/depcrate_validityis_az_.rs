// Generated macro for is_az_ (function)
macro_rules! Depcrate_validityis_az_ {
() => {
// Module: crate::validity
// Provides: {"is_az_"}
// Dependencies: {}
fn is_az_ (b : u8) -> Result < () , () > { match b { b'A' ..= b'Z' | b'a' ..= b'z' | b'_' => Ok (()) , _ => Err (()) , } }
};
}
