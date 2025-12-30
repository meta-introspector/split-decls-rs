// Generated macro for impl_145 (impl)
macro_rules! Depcrate_collections_vecimpl_145 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_145"}
// Dependencies: {}
impl < 'a , 'bump , T > Drop for Drain < 'a , 'bump , T > { fn drop (& mut self) { self . for_each (drop) ; if self . tail_len > 0 { unsafe { let source_vec = self . vec . as_mut () ; let start = source_vec . len () ; let tail = self . tail_start ; if tail != start { let src = source_vec . as_ptr () . add (tail) ; let dst = source_vec . as_mut_ptr () . add (start) ; ptr :: copy (src , dst , self . tail_len) ; } source_vec . set_len (start + self . tail_len) ; } } } }
};
}
