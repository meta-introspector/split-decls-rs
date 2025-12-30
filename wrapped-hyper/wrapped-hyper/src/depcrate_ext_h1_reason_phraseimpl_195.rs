// Generated macro for impl_195 (impl)
macro_rules! Depcrate_ext_h1_reason_phraseimpl_195 {
() => {
// Module: crate::ext::h1_reason_phrase
// Provides: {"impl_195"}
// Dependencies: {}
impl TryFrom < Bytes > for ReasonPhrase { type Error = InvalidReasonPhrase ; fn try_from (reason : Bytes) -> Result < Self , Self :: Error > { if let Some (bad_byte) = find_invalid_byte (& reason) { Err (InvalidReasonPhrase { bad_byte }) } else { Ok (Self (reason)) } } }
};
}
