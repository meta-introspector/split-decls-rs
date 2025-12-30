// Generated macro for is_valid_interface_name (function)
macro_rules! Depcrate_validityis_valid_interface_name {
() => {
// Module: crate::validity
// Provides: {"is_valid_interface_name"}
// Dependencies: {}
pub fn is_valid_interface_name (s : & [u8]) -> Result < () , () > { if s . len () > 255 { Err (()) ? } let mut x = s . into_iter () ; let mut elements = 1 ; 'outer : loop { let c = * x . next () . ok_or (()) ? ; is_az_ (c) ? ; while let Some (& c) = x . next () { if c == b'.' { elements += 1 ; continue 'outer ; } is_az09_ (c) ? ; } return if elements > 1 { Ok (()) } else { Err (()) } } }
};
}
