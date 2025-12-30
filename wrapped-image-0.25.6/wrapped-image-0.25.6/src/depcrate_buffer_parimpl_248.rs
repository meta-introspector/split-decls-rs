// Generated macro for impl_248 (impl)
macro_rules! Depcrate_buffer_parimpl_248 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_248"}
// Dependencies: {}
impl < P > fmt :: Debug for PixelsPar < '_ , P > where P : Pixel + Sync , P :: Subpixel : Sync + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("PixelsPar") . field ("chunks" , & self . chunks) . finish () } }
};
}
