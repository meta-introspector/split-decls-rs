// Generated macro for impl_126 (impl)
macro_rules! Depcrate_ext_sliceimpl_126 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'a > Iterator for Lines < 'a > { type Item = & 'a [u8] ; # [inline] fn next (& mut self) -> Option < & 'a [u8] > { Some (trim_last_terminator (self . it . next () ?)) } }
};
}
