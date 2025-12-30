// Generated macro for impl_207 (impl)
macro_rules! Depcrate_internalimpl_207 {
() => {
// Module: crate::internal
// Provides: {"impl_207"}
// Dependencies: {}
impl < T , N : ArrayLength > Drop for IntrusiveArrayBuilder < '_ , T , N > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . array . get_unchecked_mut (.. self . position) as * mut [MaybeUninit < T >] as * mut [T] ,) ; } } }
};
}
