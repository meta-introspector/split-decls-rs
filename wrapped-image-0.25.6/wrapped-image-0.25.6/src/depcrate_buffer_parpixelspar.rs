// Generated macro for PixelsPar (struct)
macro_rules! Depcrate_buffer_parPixelsPar {
() => {
// Module: crate::buffer_par
// Provides: {"PixelsPar"}
// Dependencies: {}
# [doc = " Parallel iterator over pixel refs."] # [derive (Clone)] pub struct PixelsPar < 'a , P > where P : Pixel + Sync + 'a , P :: Subpixel : Sync + 'a , { chunks : ChunksExact < 'a , P :: Subpixel > , }
};
}
