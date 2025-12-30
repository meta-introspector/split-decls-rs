// Generated macro for is_valid_unique_conn_name (function)
macro_rules! Depcrate_validityis_valid_unique_conn_name {
() => {
// Module: crate::validity
// Provides: {"is_valid_unique_conn_name"}
// Dependencies: {}
fn is_valid_unique_conn_name (mut x : std :: slice :: Iter < u8 >) -> Result < () , () > { let mut elements = 1 ; 'outer : loop { let c = * x . next () . ok_or (()) ? ; is_az09_hyphen (c) ? ; while let Some (& c) = x . next () { if c == b'.' { elements += 1 ; continue 'outer ; } is_az09_hyphen (c) ? ; } return if elements > 1 { Ok (()) } else { Err (()) } } }
};
}
