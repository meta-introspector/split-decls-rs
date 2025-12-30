// Generated macro for impl_194 (impl)
macro_rules! Depcrate_ext_h1_reason_phraseimpl_194 {
() => {
// Module: crate::ext::h1_reason_phrase
// Provides: {"impl_194"}
// Dependencies: {}
impl TryFrom < String > for ReasonPhrase { type Error = InvalidReasonPhrase ; fn try_from (reason : String) -> Result < Self , Self :: Error > { if let Some (bad_byte) = find_invalid_byte (reason . as_bytes ()) { Err (InvalidReasonPhrase { bad_byte }) } else { Ok (Self (Bytes :: from (reason))) } } }
};
}
