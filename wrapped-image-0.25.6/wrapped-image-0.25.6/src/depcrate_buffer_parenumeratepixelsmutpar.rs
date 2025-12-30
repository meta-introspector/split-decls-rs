// Generated macro for EnumeratePixelsMutPar (struct)
macro_rules! Depcrate_buffer_parEnumeratePixelsMutPar {
() => {
// Module: crate::buffer_par
// Provides: {"EnumeratePixelsMutPar"}
// Dependencies: {}
# [doc = " Parallel iterator over mutable pixel refs and their coordinates."] pub struct EnumeratePixelsMutPar < 'a , P > where P : Pixel + Send + Sync + 'a , P :: Subpixel : Send + Sync + 'a , { pixels : PixelsMutPar < 'a , P > , width : u32 , }
};
}
