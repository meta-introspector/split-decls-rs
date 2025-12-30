// Generated macro for impl_69 (impl)
macro_rules! Depcrate_iterimpl_69 {
() => {
// Module: crate::iter
// Provides: {"impl_69"}
// Dependencies: {}
impl < T , N : ArrayLength > GenericArrayIter < T , N > { # [doc = " Returns the remaining items of this iterator as a slice"] # [inline (always)] pub fn as_slice (& self) -> & [T] { unsafe { self . array . get_unchecked (self . index .. self . index_back) } } # [doc = " Returns the remaining items of this iterator as a mutable slice"] # [inline (always)] pub fn as_mut_slice (& mut self) -> & mut [T] { unsafe { self . array . get_unchecked_mut (self . index .. self . index_back) } } }
};
}
