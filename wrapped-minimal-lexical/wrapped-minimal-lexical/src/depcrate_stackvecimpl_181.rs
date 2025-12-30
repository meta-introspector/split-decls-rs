// Generated macro for impl_181 (impl)
macro_rules! Depcrate_stackvecimpl_181 {
() => {
// Module: crate::stackvec
// Provides: {"impl_181"}
// Dependencies: {}
impl ops :: Deref for StackVec { type Target = [bigint :: Limb] ; # [inline] fn deref (& self) -> & [bigint :: Limb] { unsafe { let ptr = self . data . as_ptr () as * const bigint :: Limb ; slice :: from_raw_parts (ptr , self . len ()) } } }
};
}
