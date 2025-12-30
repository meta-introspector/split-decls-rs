// Generated macro for is_valid_signature_multi (function)
macro_rules! Depcrate_validityis_valid_signature_multi {
() => {
// Module: crate::validity
// Provides: {"is_valid_signature_multi"}
// Dependencies: {}
pub fn is_valid_signature_multi (s : & [u8]) -> Result < () , () > { if s . len () > 255 { Err (()) ? } let pos = sig_multi (s , 0 , 0) . ok_or (()) ? ; return if pos == s . len () { Ok (()) } else { Err (()) } }
};
}
