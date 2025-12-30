// Generated macro for is_valid_string (function)
macro_rules! Depcrate_validityis_valid_string {
() => {
// Module: crate::validity
// Provides: {"is_valid_string"}
// Dependencies: {}
pub fn is_valid_string (s : & str) -> Result < () , () > { let s = s . as_bytes () ; if s . len () >= 134217728 { Err (()) } else if s . iter () . any (| & b | b == 0) { Err (()) } else { Ok (()) } }
};
}
