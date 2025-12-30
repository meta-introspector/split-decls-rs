// Generated macro for impl_477 (impl)
macro_rules! Depcrate_meta_wrappersimpl_477 {
() => {
// Module: crate::meta::wrappers
// Provides: {"impl_477"}
// Dependencies: {}
impl BoundedBacktrackerCache { pub (crate) fn none () -> BoundedBacktrackerCache { # [cfg (feature = "nfa-backtrack")] { BoundedBacktrackerCache (None) } # [cfg (not (feature = "nfa-backtrack"))] { BoundedBacktrackerCache (()) } } pub (crate) fn reset (& mut self , builder : & BoundedBacktracker) { # [cfg (feature = "nfa-backtrack")] if let Some (ref e) = builder . 0 { self . get (& e . 0) . reset (& e . 0) ; } } pub (crate) fn memory_usage (& self) -> usize { # [cfg (feature = "nfa-backtrack")] { self . 0 . as_ref () . map_or (0 , | c | c . memory_usage ()) } # [cfg (not (feature = "nfa-backtrack"))] { 0 } } # [cfg (feature = "nfa-backtrack")] fn get (& mut self , bb : & backtrack :: BoundedBacktracker ,) -> & mut backtrack :: Cache { self . 0 . get_or_insert_with (| | bb . create_cache ()) } }
};
}
