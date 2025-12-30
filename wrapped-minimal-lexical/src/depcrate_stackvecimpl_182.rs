// Generated macro for impl_182 (impl)
macro_rules! Depcrate_stackvecimpl_182 {
() => {
// Module: crate::stackvec
// Provides: {"impl_182"}
// Dependencies: {}
impl ops :: DerefMut for StackVec { # [inline] fn deref_mut (& mut self) -> & mut [bigint :: Limb] { unsafe { let ptr = self . data . as_mut_ptr () as * mut bigint :: Limb ; slice :: from_raw_parts_mut (ptr , self . len ()) } } }
};
}
