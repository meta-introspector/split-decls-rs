// Generated macro for SizeDesc (trait)
macro_rules! Depcrate_style_sizeSizeDesc {
() => {
// Module: crate::style::size
// Provides: {"SizeDesc"}
// Dependencies: {}
# [doc = " The trait that describes a size, it may be a relative size which the"] # [doc = " size is determined by the parent size, e.g., 10% of the parent width"] pub trait SizeDesc { # [doc = " Convert the size into the number of pixels"] # [doc = ""] # [doc = " - `parent`: The reference to the parent container of this size"] # [doc = " - **returns**: The number of pixels"] fn in_pixels < T : HasDimension > (& self , parent : & T) -> i32 ; }
};
}
