// Generated macro for SubTCategoryOf (trait)
macro_rules! Depcrate_geometry_transformSubTCategoryOf {
() => {
// Module: crate::geometry::transform
// Provides: {"SubTCategoryOf"}
// Dependencies: {}
# [doc = " Indicates that `Self` is a more specific `Transform` category than `Other`."] # [doc = ""] # [doc = " Automatically implemented based on `SuperTCategoryOf`."] pub trait SubTCategoryOf < Other : TCategory > : TCategory { }
};
}
