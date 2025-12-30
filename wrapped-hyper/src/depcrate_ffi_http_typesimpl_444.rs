// Generated macro for impl_444 (impl)
macro_rules! Depcrate_ffi_http_typesimpl_444 {
() => {
// Module: crate::ffi::http_types
// Provides: {"impl_444"}
// Dependencies: {}
impl hyper_headers { pub (super) fn get_or_default (ext : & mut http :: Extensions) -> & mut hyper_headers { if let None = ext . get_mut :: < hyper_headers > () { ext . insert (hyper_headers :: default ()) ; } ext . get_mut :: < hyper_headers > () . unwrap () } }
};
}
