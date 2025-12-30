// Generated macro for impl_167 (impl)
macro_rules! Depcrate_easy_listimpl_167 {
() => {
// Module: crate::easy::list
// Provides: {"impl_167"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = & 'a [u8] ; fn next (& mut self) -> Option < & 'a [u8] > { if self . cur . is_null () { return None ; } unsafe { let ret = Some (CStr :: from_ptr ((* self . cur) . data) . to_bytes ()) ; self . cur = (* self . cur) . next ; ret } } }
};
}
