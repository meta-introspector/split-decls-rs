// Generated macro for impl_193 (impl)
macro_rules! Depcrate_ext_h1_reason_phraseimpl_193 {
() => {
// Module: crate::ext::h1_reason_phrase
// Provides: {"impl_193"}
// Dependencies: {}
impl TryFrom < Vec < u8 > > for ReasonPhrase { type Error = InvalidReasonPhrase ; fn try_from (reason : Vec < u8 >) -> Result < Self , Self :: Error > { if let Some (bad_byte) = find_invalid_byte (& reason) { Err (InvalidReasonPhrase { bad_byte }) } else { Ok (Self (Bytes :: from (reason))) } } }
};
}
