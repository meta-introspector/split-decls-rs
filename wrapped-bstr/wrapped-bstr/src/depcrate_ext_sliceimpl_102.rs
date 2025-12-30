// Generated macro for impl_102 (impl)
macro_rules! Depcrate_ext_sliceimpl_102 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'a > Iterator for Bytes < 'a > { type Item = u8 ; # [inline] fn next (& mut self) -> Option < u8 > { self . it . next () . copied () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
