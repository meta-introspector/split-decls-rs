// Generated macro for impl_433 (impl)
macro_rules! Depcrate_ffi_http_typesimpl_433 {
() => {
// Module: crate::ffi::http_types
// Provides: {"impl_433"}
// Dependencies: {}
impl hyper_request { pub (super) fn finalize_request (& mut self) { if let Some (headers) = self . 0 . extensions_mut () . remove :: < hyper_headers > () { * self . 0 . headers_mut () = headers . headers ; self . 0 . extensions_mut () . insert (headers . orig_casing) ; self . 0 . extensions_mut () . insert (headers . orig_order) ; } } }
};
}
