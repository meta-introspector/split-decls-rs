// Generated macro for is_valid_member_name (function)
macro_rules! Depcrate_validityis_valid_member_name {
() => {
// Module: crate::validity
// Provides: {"is_valid_member_name"}
// Dependencies: {}
pub fn is_valid_member_name (s : & [u8]) -> Result < () , () > { if s . len () > 255 { Err (()) ? } let mut x = s . into_iter () ; let c = * x . next () . ok_or (()) ? ; is_az_ (c) ? ; for c in x { is_az09_ (* c) ? } ; Ok (()) }
};
}
