// Generated macro for SliceRange (trait)
macro_rules! Depcrate_base_matrix_viewSliceRange {
() => {
// Module: crate::base::matrix_view
// Provides: {"SliceRange"}
// Dependencies: {}
# [doc = " A range with a size that may be known at compile-time."] # [doc = ""] # [doc = " This is merely a legacy trait alias to minimize breakage. Use the [`DimRange`] trait instead."] # [deprecated = slice_deprecation_note ! (DimRange)] pub trait SliceRange < D : Dim > : DimRange < D > { }
};
}
