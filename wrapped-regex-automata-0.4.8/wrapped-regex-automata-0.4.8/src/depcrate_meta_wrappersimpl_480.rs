// Generated macro for impl_480 (impl)
macro_rules! Depcrate_meta_wrappersimpl_480 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_480"}
// Dependencies: {}
impl OnePassCache { pub (crate) fn none () -> OnePassCache { # [cfg (feature = "dfa-onepass")] { OnePassCache (None) } # [cfg (not (feature = "dfa-onepass"))] { OnePassCache (()) } } pub (crate) fn new (builder : & OnePass) -> OnePassCache { # [cfg (feature = "dfa-onepass")] { OnePassCache (builder . 0 . as_ref () . map (| e | e . 0 . create_cache ())) } # [cfg (not (feature = "dfa-onepass"))] { OnePassCache (()) } } pub (crate) fn reset (& mut self , builder : & OnePass) { # [cfg (feature = "dfa-onepass")] if let Some (ref e) = builder . 0 { self . 0 . as_mut () . unwrap () . reset (& e . 0) ; } } pub (crate) fn memory_usage (& self) -> usize { # [cfg (feature = "dfa-onepass")] { self . 0 . as_ref () . map_or (0 , | c | c . memory_usage ()) } # [cfg (not (feature = "dfa-onepass"))] { 0 } } }
};
}
