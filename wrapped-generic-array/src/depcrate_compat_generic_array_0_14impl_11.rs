// Generated macro for impl_11 (impl)
macro_rules! Depcrate_compat_generic_array_0_14impl_11 {
() => {
// Module: crate::compat::generic_array_0_14
// Provides: {"impl_11"}
// Dependencies: {}
impl < T , N : ArrayLength + ArrayLength_0_14 < T > > GenericArray < T , N > { # [doc = " From `&self` of this version, create a reference to the [`GenericArray`](GenericArray_0_14) type from `generic-array` 0.14."] # [inline (always)] pub const fn as_0_14 (& self) -> & GenericArray_0_14 < T , N > { unsafe { core :: mem :: transmute (self) } } # [doc = " From `&mut self` of this version, create a mutable reference to the [`GenericArray`](GenericArray_0_14) type from `generic-array` 0.14."] # [doc = ""] # [doc = " This method is `const` since Rust 1.83.0, but non-`const` before."] # [rustversion :: attr (since (1.83) , const)] # [inline (always)] pub fn as_0_14_mut (& mut self) -> & mut GenericArray_0_14 < T , N > { unsafe { core :: mem :: transmute (self) } } # [doc = " From `self` of this version, create the [`GenericArray`](GenericArray_0_14) type from `generic-array` 0.14."] # [inline (always)] pub const fn into_0_14 (self) -> GenericArray_0_14 < T , N > { unsafe { crate :: const_transmute (self) } } # [doc = " From the [`GenericArray`](GenericArray_0_14) type from `generic-array` 0.14, create a [`GenericArray`] of this version."] # [inline (always)] pub const fn from_0_14 (value : GenericArray_0_14 < T , N >) -> Self { unsafe { crate :: const_transmute (value) } } }
};
}
