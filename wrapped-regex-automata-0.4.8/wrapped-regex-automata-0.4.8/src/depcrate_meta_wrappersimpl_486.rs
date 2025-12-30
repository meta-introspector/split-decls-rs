// Generated macro for impl_486 (impl)
macro_rules! Depcrate_meta_wrappersimpl_486 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_486"}
// Dependencies: {}
impl HybridCache { pub (crate) fn none () -> HybridCache { # [cfg (feature = "hybrid")] { HybridCache (None) } # [cfg (not (feature = "hybrid"))] { HybridCache (()) } } pub (crate) fn new (builder : & Hybrid) -> HybridCache { # [cfg (feature = "hybrid")] { HybridCache (builder . 0 . as_ref () . map (| e | e . 0 . create_cache ())) } # [cfg (not (feature = "hybrid"))] { HybridCache (()) } } pub (crate) fn reset (& mut self , builder : & Hybrid) { # [cfg (feature = "hybrid")] if let Some (ref e) = builder . 0 { self . 0 . as_mut () . unwrap () . reset (& e . 0) ; } } pub (crate) fn memory_usage (& self) -> usize { # [cfg (feature = "hybrid")] { self . 0 . as_ref () . map_or (0 , | c | c . memory_usage ()) } # [cfg (not (feature = "hybrid"))] { 0 } } }
};
}
