// Generated macro for IntoPartialRef (trait)
macro_rules! DepcrateIntoPartialRef {
() => {
// Module: crate
// Provides: {"IntoPartialRef"}
// Dependencies: {}
# [doc = " Construction of partial references."] # [doc = ""] # [doc = " See also [`IntoPartialRefMut`]."] # [doc = ""] # [doc = " This trait gets an automatic implementation for references (mutable or immutable) to any type"] # [doc = " that has a derive statement for [`PartialRefTarget`]. Usually there is no need to implement this"] # [doc = " trait manually."] pub trait IntoPartialRef < 'a > { type Ref : PartialRef < 'a > ; # [doc = " Convert a mutable or immutable reference into a partial reference."] fn into_partial_ref (self) -> Self :: Ref ; }
};
}
