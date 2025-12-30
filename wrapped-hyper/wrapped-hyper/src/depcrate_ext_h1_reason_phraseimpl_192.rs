// Generated macro for impl_192 (impl)
macro_rules! Depcrate_ext_h1_reason_phraseimpl_192 {
() => {
// Module: crate::ext::h1_reason_phrase
// Provides: {"impl_192"}
// Dependencies: {}
impl TryFrom < & [u8] > for ReasonPhrase { type Error = InvalidReasonPhrase ; fn try_from (reason : & [u8]) -> Result < Self , Self :: Error > { if let Some (bad_byte) = find_invalid_byte (reason) { Err (InvalidReasonPhrase { bad_byte }) } else { Ok (Self (Bytes :: copy_from_slice (reason))) } } }
};
}
