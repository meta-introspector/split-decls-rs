// Generated macro for impl_499 (impl)
macro_rules! Depcrate_meta_wrappersimpl_499 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_499"}
// Dependencies: {}
impl ReverseHybridCache { pub (crate) fn none () -> ReverseHybridCache { # [cfg (feature = "hybrid")] { ReverseHybridCache (None) } # [cfg (not (feature = "hybrid"))] { ReverseHybridCache (()) } } pub (crate) fn new (builder : & ReverseHybrid) -> ReverseHybridCache { # [cfg (feature = "hybrid")] { ReverseHybridCache (builder . 0 . as_ref () . map (| e | e . 0 . create_cache ())) } # [cfg (not (feature = "hybrid"))] { ReverseHybridCache (()) } } pub (crate) fn reset (& mut self , builder : & ReverseHybrid) { # [cfg (feature = "hybrid")] if let Some (ref e) = builder . 0 { self . 0 . as_mut () . unwrap () . reset (& e . 0) ; } } pub (crate) fn memory_usage (& self) -> usize { # [cfg (feature = "hybrid")] { self . 0 . as_ref () . map_or (0 , | c | c . memory_usage ()) } # [cfg (not (feature = "hybrid"))] { 0 } } }
};
}
