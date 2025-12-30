// Generated macro for impl_79 (impl)
macro_rules! Depcrate_commitimpl_79 {
() => {
// Module: crate::commit
// Provides: {"impl_79"}
// Dependencies: {}
impl Commit { # [doc = " Returns a convenient iterator over all extra headers."] pub fn extra_headers (& self) -> ExtraHeaders < impl Iterator < Item = (& BStr , & BStr) > > { ExtraHeaders :: new (self . extra_headers . iter () . map (| (k , v) | (k . as_bstr () , v . as_bstr ()))) } }
};
}
