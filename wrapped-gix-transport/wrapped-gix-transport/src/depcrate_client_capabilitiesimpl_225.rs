// Generated macro for impl_225 (impl)
macro_rules! Depcrate_client_capabilitiesimpl_225 {
() => {
// Module: crate::client::capabilities
// Provides: {"impl_225"}
// Dependencies: {}
impl < 'a > Capability < 'a > { # [doc = " Returns the name of the capability."] # [doc = ""] # [doc = " Most capabilities only consist of a name, making them appear like a feature toggle."] pub fn name (& self) -> & 'a BStr { self . 0 . splitn (2 , | b | * b == b'=') . next () . expect ("there is always a single item") . as_bstr () } # [doc = " Returns the value associated with the capability."] # [doc = ""] # [doc = " Note that the caller must know whether a single or multiple values are expected, in which"] # [doc = " case [`values()`](Capability::values()) should be called."] pub fn value (& self) -> Option < & 'a BStr > { self . 0 . splitn (2 , | b | * b == b'=') . nth (1) . map (ByteSlice :: as_bstr) } # [doc = " Returns the values of a capability if its [`value()`](Capability::value()) is space separated."] pub fn values (& self) -> Option < impl Iterator < Item = & 'a BStr > > { self . value () . map (| v | v . split (| b | * b == b' ') . map (ByteSlice :: as_bstr)) } # [doc = " Returns true if its space-separated [`value()`](Capability::value()) contains the given `want`ed capability."] pub fn supports (& self , want : impl Into < & 'a BStr >) -> Option < bool > { let want = want . into () ; self . values () . map (| mut iter | iter . any (| v | v == want)) } }
};
}
