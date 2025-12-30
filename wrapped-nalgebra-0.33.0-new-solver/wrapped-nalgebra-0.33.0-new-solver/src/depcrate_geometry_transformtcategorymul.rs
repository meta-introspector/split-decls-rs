// Generated macro for TCategoryMul (trait)
macro_rules! Depcrate_geometry_transformTCategoryMul {
() => {
// Module: crate::geometry::transform
// Provides: {"TCategoryMul"}
// Dependencies: {}
# [doc = " Traits that gives the `Transform` category that is compatible with the result of the"] # [doc = " multiplication of transformations with categories `Self` and `Other`."] pub trait TCategoryMul < Other : TCategory > : TCategory { # [doc = " The transform category that results from the multiplication of a `Transform<Self>` to a"] # [doc = " `Transform<Other>`. This is usually equal to `Self` or `Other`, whichever is the most"] # [doc = " general category."] type Representative : TCategory ; }
};
}
