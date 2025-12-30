// Generated macro for impl_256 (impl)
macro_rules! Depcrate_buffer_parimpl_256 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_256"}
// Dependencies: {}
impl < P > fmt :: Debug for EnumeratePixelsPar < '_ , P > where P : Pixel + Sync , P :: Subpixel : Sync + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("EnumeratePixelsPar") . field ("pixels" , & self . pixels) . field ("width" , & self . width) . finish () } }
};
}
