// Generated macro for is_valid_signature_single (function)
macro_rules! Depcrate_validityis_valid_signature_single {
() => {
// Module: crate::validity
// Provides: {"is_valid_signature_single"}
// Dependencies: {}
pub fn is_valid_signature_single (s : & [u8]) -> Result < () , () > { if s . len () > 255 { Err (()) ? } let pos = sig_single (s , 0 , 0) . ok_or (()) ? ; return if pos == s . len () { Ok (()) } else { Err (()) } }
};
}
