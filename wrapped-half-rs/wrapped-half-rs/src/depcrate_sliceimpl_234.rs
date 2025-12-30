// Generated macro for impl_234 (impl)
macro_rules! Depcrate_sliceimpl_234 {
() => {
// Module: crate::slice
// Provides: {"impl_234"}
// Dependencies: {}
impl HalfBitsSliceExt for [u16] { # [inline] fn reinterpret_cast < H > (& self) -> & [H] where H : crate :: private :: SealedHalf , { transmute_ref ! (self) } # [inline] fn reinterpret_cast_mut < H > (& mut self) -> & mut [H] where H : crate :: private :: SealedHalf , { transmute_mut ! (self) } }
};
}
