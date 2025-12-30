// Generated macro for impl_1268 (impl)
macro_rules! Depcrate_style_sizeimpl_1268 {
() => {
// Module: crate::style::size
// Provides: {"impl_1268"}
// Dependencies: {}
impl SizeDesc for RelativeSizeWithBound { fn in_pixels < D : HasDimension > (& self , parent : & D) -> i32 { let size = self . size . in_pixels (parent) ; let size_lower_capped = self . min . map_or (size , | x | x . max (size)) ; self . max . map_or (size_lower_capped , | x | x . min (size)) } }
};
}
