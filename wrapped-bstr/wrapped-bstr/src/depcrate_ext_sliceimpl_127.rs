// Generated macro for impl_127 (impl)
macro_rules! Depcrate_ext_sliceimpl_127 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_127"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Lines < 'a > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { Some (trim_last_terminator (self . it . next_back () ?)) } }
};
}
