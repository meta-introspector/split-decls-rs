// Generated macro for impl_260 (impl)
macro_rules! Depcrate_buffer_parimpl_260 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_260"}
// Dependencies: {}
impl < P > fmt :: Debug for EnumeratePixelsMutPar < '_ , P > where P : Pixel + Send + Sync , P :: Subpixel : Send + Sync + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("EnumeratePixelsMutPar") . field ("pixels" , & self . pixels) . field ("width" , & self . width) . finish () } }
};
}
