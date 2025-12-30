// Generated macro for impl_495 (impl)
macro_rules! Depcrate_meta_wrappersimpl_495 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_495"}
// Dependencies: {}
impl ReverseHybrid { pub (crate) fn none () -> ReverseHybrid { ReverseHybrid (None) } pub (crate) fn new (info : & RegexInfo , nfarev : & NFA) -> ReverseHybrid { ReverseHybrid (ReverseHybridEngine :: new (info , nfarev)) } pub (crate) fn create_cache (& self) -> ReverseHybridCache { ReverseHybridCache :: new (self) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn get (& self , _input : & Input < '_ > ,) -> Option < & ReverseHybridEngine > { let engine = self . 0 . as_ref () ? ; Some (engine) } }
};
}
