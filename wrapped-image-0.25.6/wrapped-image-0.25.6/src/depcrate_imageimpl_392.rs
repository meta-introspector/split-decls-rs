// Generated macro for impl_392 (impl)
macro_rules! Depcrate_imageimpl_392 {
() => {
// Module: crate::image
// Provides: {"impl_392"}
// Dependencies: {}
impl < I : GenericImageView > Iterator for Pixels < '_ , I > { type Item = (u32 , u32 , I :: Pixel) ; fn next (& mut self) -> Option < (u32 , u32 , I :: Pixel) > { if self . x >= self . width { self . x = 0 ; self . y += 1 ; } if self . y >= self . height { None } else { let pixel = self . image . get_pixel (self . x , self . y) ; let p = (self . x , self . y , pixel) ; self . x += 1 ; Some (p) } } }
};
}
