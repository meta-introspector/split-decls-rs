// Generated macro for is_valid_object_path (function)
macro_rules! Depcrate_validityis_valid_object_path {
() => {
// Module: crate::validity
// Provides: {"is_valid_object_path"}
// Dependencies: {}
pub fn is_valid_object_path (s : & [u8]) -> Result < () , () > { let mut x = s . into_iter () ; let c = x . next () ; if c != Some (& b'/') { Err (()) ? } ; if s . len () == 1 { return Ok (()) } ; 'outer : loop { let c = * x . next () . ok_or (()) ? ; is_az09_ (c) ? ; while let Some (& c) = x . next () { if c == b'/' { continue 'outer ; } is_az09_ (c) ? ; } return Ok (()) ; } }
};
}
