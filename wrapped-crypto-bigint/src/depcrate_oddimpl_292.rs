// Generated macro for impl_292 (impl)
macro_rules! Depcrate_oddimpl_292 {
() => {
// Module: crate::odd
// Provides: {"impl_292"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Odd < BoxedUint > { # [doc = " Borrow the limbs of this [`Odd<BoxedUint>`] as a [`Odd<UintRef>`]."] pub (crate) const fn as_uint_ref (& self) -> & Odd < UintRef > { # [allow (trivial_casts , unsafe_code)] unsafe { & * (self . 0 . as_uint_ref () as * const UintRef as * const Odd < UintRef >) } } }
};
}
