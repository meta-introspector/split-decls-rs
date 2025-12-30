// Generated macro for impl_1263 (impl)
macro_rules! Depcrate_style_sizeimpl_1263 {
() => {
// Module: crate::style::size
// Provides: {"impl_1263"}
// Dependencies: {}
impl SizeDesc for RelativeSize { fn in_pixels < D : HasDimension > (& self , parent : & D) -> i32 { let (w , h) = parent . dim () ; match self { RelativeSize :: Width (p) => * p * f64 :: from (w) , RelativeSize :: Height (p) => * p * f64 :: from (h) , RelativeSize :: Smaller (p) => * p * f64 :: from (w . min (h)) , } . round () as i32 } }
};
}
