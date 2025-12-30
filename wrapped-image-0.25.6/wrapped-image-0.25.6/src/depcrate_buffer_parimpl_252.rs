// Generated macro for impl_252 (impl)
macro_rules! Depcrate_buffer_parimpl_252 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_252"}
// Dependencies: {}
impl < P > fmt :: Debug for PixelsMutPar < '_ , P > where P : Pixel + Send + Sync , P :: Subpixel : Send + Sync + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("PixelsMutPar") . field ("chunks" , & self . chunks) . finish () } }
};
}
