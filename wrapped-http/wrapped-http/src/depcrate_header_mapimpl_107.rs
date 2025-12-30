// Generated macro for impl_107 (impl)
macro_rules! Depcrate_header_mapimpl_107 {
() => {
// Module: crate::header::map
// Provides: {"impl_107"}
// Dependencies: {}
impl < T > Drop for IntoIter < T > { fn drop (& mut self) { for _ in self . by_ref () { } unsafe { self . extra_values . set_len (0) ; } } }
};
}
