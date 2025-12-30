// Generated macro for impl_441 (impl)
macro_rules! Depcrate_ffi_http_typesimpl_441 {
() => {
// Module: crate::ffi::http_types
// Provides: {"impl_441"}
// Dependencies: {}
impl hyper_response { pub (super) fn wrap (mut resp : Response < IncomingBody >) -> hyper_response { let headers = std :: mem :: take (resp . headers_mut ()) ; let orig_casing = resp . extensions_mut () . remove :: < HeaderCaseMap > () . unwrap_or_else (HeaderCaseMap :: default) ; let orig_order = resp . extensions_mut () . remove :: < OriginalHeaderOrder > () . unwrap_or_else (OriginalHeaderOrder :: default) ; resp . extensions_mut () . insert (hyper_headers { headers , orig_casing , orig_order , }) ; hyper_response (resp) } fn reason_phrase (& self) -> & [u8] { if let Some (reason) = self . 0 . extensions () . get :: < ReasonPhrase > () { return reason . as_bytes () ; } if let Some (reason) = self . 0 . status () . canonical_reason () { return reason . as_bytes () ; } & [] } }
};
}
