// Generated macro for impl_1267 (impl)
macro_rules! Depcrate_style_sizeimpl_1267 {
() => {
// Module: crate::style::size
// Provides: {"impl_1267"}
// Dependencies: {}
impl RelativeSizeWithBound { # [doc = " Set the lower bound of the bounded relative size"] # [doc = ""] # [doc = " - `min_sz`: The lower bound of this size description"] # [doc = " - **returns**: The newly created size description with the bound"] pub fn min (mut self , min_sz : i32) -> RelativeSizeWithBound { self . min = Some (min_sz) ; self } # [doc = " Set the upper bound of the bounded relative size"] # [doc = ""] # [doc = " - `min_sz`: The upper bound of this size description"] # [doc = " - **returns**: The newly created size description with the bound"] pub fn max (mut self , max_sz : i32) -> RelativeSizeWithBound { self . max = Some (max_sz) ; self } }
};
}
