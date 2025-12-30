// Generated macro for impl_474 (impl)
macro_rules! Depcrate_meta_wrappersimpl_474 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_474"}
// Dependencies: {}
impl BoundedBacktrackerCache { pub (crate) fn none () -> BoundedBacktrackerCache { # [cfg (feature = "nfa-backtrack")] { BoundedBacktrackerCache (None) } # [cfg (not (feature = "nfa-backtrack"))] { BoundedBacktrackerCache (()) } } pub (crate) fn new (builder : & BoundedBacktracker ,) -> BoundedBacktrackerCache { # [cfg (feature = "nfa-backtrack")] { BoundedBacktrackerCache (builder . 0 . as_ref () . map (| e | e . 0 . create_cache ()) ,) } # [cfg (not (feature = "nfa-backtrack"))] { BoundedBacktrackerCache (()) } } pub (crate) fn reset (& mut self , builder : & BoundedBacktracker) { # [cfg (feature = "nfa-backtrack")] if let Some (ref e) = builder . 0 { self . 0 . as_mut () . unwrap () . reset (& e . 0) ; } } pub (crate) fn memory_usage (& self) -> usize { # [cfg (feature = "nfa-backtrack")] { self . 0 . as_ref () . map_or (0 , | c | c . memory_usage ()) } # [cfg (not (feature = "nfa-backtrack"))] { 0 } } }
};
}
