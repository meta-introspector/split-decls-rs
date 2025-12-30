// Generated macro for PixelsMutPar (struct)
macro_rules! Depcrate_buffer_parPixelsMutPar {
() => {
// Module: crate::buffer_par
// Provides: {"PixelsMutPar"}
// Dependencies: {}
# [doc = " Parallel iterator over mutable pixel refs."] pub struct PixelsMutPar < 'a , P > where P : Pixel + Send + Sync + 'a , P :: Subpixel : Send + Sync + 'a , { chunks : ChunksExactMut < 'a , P :: Subpixel > , }
};
}
