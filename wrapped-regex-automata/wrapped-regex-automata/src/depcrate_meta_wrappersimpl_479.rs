// Generated macro for impl_479 (impl)
macro_rules! Depcrate_meta_wrappersimpl_479 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_479"}
// Dependencies: {}
impl OnePass { pub (crate) fn new (info : & RegexInfo , nfa : & NFA) -> OnePass { OnePass (OnePassEngine :: new (info , nfa)) } pub (crate) fn create_cache (& self) -> OnePassCache { OnePassCache :: new (self) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn get (& self , input : & Input < '_ >) -> Option < & OnePassEngine > { let engine = self . 0 . as_ref () ? ; if ! input . get_anchored () . is_anchored () && ! engine . get_nfa () . is_always_start_anchored () { return None ; } Some (engine) } pub (crate) fn memory_usage (& self) -> usize { self . 0 . as_ref () . map_or (0 , | e | e . memory_usage ()) } }
};
}
