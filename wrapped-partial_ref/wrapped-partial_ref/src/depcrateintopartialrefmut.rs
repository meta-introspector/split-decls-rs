// Generated macro for IntoPartialRefMut (trait)
macro_rules! DepcrateIntoPartialRefMut {
() => {
// Module: crate
// Provides: {"IntoPartialRefMut"}
// Dependencies: {}
# [doc = " Construction of partial references from mutable references."] # [doc = ""] # [doc = " This has an implementation for mutable references that implement [`IntoPartialRefMut`]. It"] # [doc = " performs the same operation as [`IntoPartialRefMut`] but is only implemented for mutable"] # [doc = " references. This is useful as it allows writing `value.into_partial_ref_mut()` instead of `(&mut"] # [doc = " value).into_partial_ref_mut()` using auto referencing of method calls. Using just"] # [doc = " [`value.into_partial_ref()`] would result in an immutable reference."] pub trait IntoPartialRefMut < 'a > : IntoPartialRef < 'a > { # [doc = " Convert a mutable reference into a partial reference."] fn into_partial_ref_mut (self) -> Self :: Ref ; }
};
}
