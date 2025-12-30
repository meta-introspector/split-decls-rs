// Generated macro for impl_204 (impl)
macro_rules! Depcrate_internalimpl_204 {
() => {
// Module: crate::internal
// Provides: {"impl_204"}
// Dependencies: {}
impl < T , N : ArrayLength > Drop for ArrayBuilder < T , N > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . array . get_unchecked_mut (.. self . position) as * mut [MaybeUninit < T >] as * mut [T] ,) ; } } }
};
}
