// Generated macro for impl_275 (impl)
macro_rules! Depcrate_oddimpl_275 {
() => {
// Module: crate::odd
// Provides: {"impl_275"}
// Dependencies: {}
impl < T : ? Sized > Odd < T > { # [doc = " Provides access to the contents of [`Odd`] in a `const` context."] pub const fn as_ref (& self) -> & T { & self . 0 } # [doc = " All odd integers are definitionally non-zero, so we can also obtain a reference to [`NonZero`]."] pub const fn as_nz_ref (& self) -> & NonZero < T > { # [allow (trivial_casts , unsafe_code)] unsafe { & * (& self . 0 as * const T as * const NonZero < T >) } } }
};
}
