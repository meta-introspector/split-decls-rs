// Generated macro for impl_253 (impl)
macro_rules! Depcrate_hkdfimpl_253 {
() => {
// Module: crate::hkdf
// Provides: {"impl_253"}
// Dependencies: {}
impl < T : Clone + Zeroize > From < & [T] > for ZeroizeBoxSlice < T > { fn from (value : & [T]) -> Self { Self (Vec :: from (value) . into_boxed_slice ()) } }
};
}
