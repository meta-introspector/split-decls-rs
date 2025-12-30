// Generated macro for impl_191 (impl)
macro_rules! Depcrate_ext_h1_reason_phraseimpl_191 {
() => {
// Module: crate::ext::h1_reason_phrase
// Provides: {"impl_191"}
// Dependencies: {}
impl ReasonPhrase { # [doc = " Gets the reason phrase as bytes."] pub fn as_bytes (& self) -> & [u8] { & self . 0 } # [doc = " Converts a static byte slice to a reason phrase."] pub const fn from_static (reason : & 'static [u8]) -> Self { if find_invalid_byte (reason) . is_some () { panic ! ("invalid byte in static reason phrase") ; } Self (Bytes :: from_static (reason)) } # [doc = " Converts a `Bytes` directly into a `ReasonPhrase` without validating."] # [doc = ""] # [doc = " Use with care; invalid bytes in a reason phrase can cause serious security problems if"] # [doc = " emitted in a response."] # [cfg (feature = "client")] pub (crate) fn from_bytes_unchecked (reason : Bytes) -> Self { Self (reason) } }
};
}
