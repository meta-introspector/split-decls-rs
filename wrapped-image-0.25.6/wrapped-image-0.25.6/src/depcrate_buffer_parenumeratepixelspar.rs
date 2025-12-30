// Generated macro for EnumeratePixelsPar (struct)
macro_rules! Depcrate_buffer_parEnumeratePixelsPar {
() => {
// Module: crate::buffer_par
// Provides: {"EnumeratePixelsPar"}
// Dependencies: {}
# [doc = " Parallel iterator over pixel refs and their coordinates."] # [derive (Clone)] pub struct EnumeratePixelsPar < 'a , P > where P : Pixel + Sync + 'a , P :: Subpixel : Sync + 'a , { pixels : PixelsPar < 'a , P > , width : u32 , }
};
}
