// Generated macro for is_valid_bus_name (function)
macro_rules! Depcrate_validityis_valid_bus_name {
() => {
// Module: crate::validity
// Provides: {"is_valid_bus_name"}
// Dependencies: {}
pub fn is_valid_bus_name (s : & [u8]) -> Result < () , () > { if s . len () > 255 { return Err (()) ; } let mut x = s . into_iter () ; let mut c_first = * x . next () . ok_or (()) ? ; if c_first == b':' { return is_valid_unique_conn_name (x) ; } let mut elements = 1 ; 'outer : loop { is_az_hyphen (c_first) ? ; while let Some (& c) = x . next () { if c == b'.' { elements += 1 ; c_first = * x . next () . ok_or (()) ? ; continue 'outer ; } is_az09_hyphen (c) ? ; } return if elements > 1 { Ok (()) } else { Err (()) } } }
};
}
