// Generated macro for impl_1262 (impl)
macro_rules! Depcrate_style_sizeimpl_1262 {
() => {
// Module: crate::style::size
// Provides: {"impl_1262"}
// Dependencies: {}
impl RelativeSize { # [doc = " Set the lower bound of the relative size."] # [doc = ""] # [doc = " - `min_sz`: The minimal size the relative size can be in pixels"] # [doc = " - **returns**: The relative size with the bound"] pub fn min (self , min_sz : i32) -> RelativeSizeWithBound { RelativeSizeWithBound { size : self , min : Some (min_sz) , max : None , } } # [doc = " Set the upper bound of the relative size"] # [doc = ""] # [doc = " - `max_size`: The maximum size in pixels for this relative size"] # [doc = " - **returns** The relative size with the upper bound"] pub fn max (self , max_sz : i32) -> RelativeSizeWithBound { RelativeSizeWithBound { size : self , max : Some (max_sz) , min : None , } } }
};
}
