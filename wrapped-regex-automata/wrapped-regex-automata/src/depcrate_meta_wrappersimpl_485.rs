// Generated macro for impl_485 (impl)
macro_rules! Depcrate_meta_wrappersimpl_485 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_485"}
// Dependencies: {}
impl Hybrid { pub (crate) fn none () -> Hybrid { Hybrid (None) } pub (crate) fn new (info : & RegexInfo , pre : Option < Prefilter > , nfa : & NFA , nfarev : & NFA ,) -> Hybrid { Hybrid (HybridEngine :: new (info , pre , nfa , nfarev)) } pub (crate) fn create_cache (& self) -> HybridCache { HybridCache :: new (self) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn get (& self , _input : & Input < '_ >) -> Option < & HybridEngine > { let engine = self . 0 . as_ref () ? ; Some (engine) } pub (crate) fn is_some (& self) -> bool { self . 0 . is_some () } }
};
}
