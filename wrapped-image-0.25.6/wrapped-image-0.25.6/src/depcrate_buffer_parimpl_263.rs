// Generated macro for impl_263 (impl)
macro_rules! Depcrate_buffer_parimpl_263 {
() => {
// Module: crate::buffer_par
// Provides: {"impl_263"}
// Dependencies: {}
impl < P > ImageBuffer < P , Vec < P :: Subpixel > > where P : Pixel + Send + Sync , P :: Subpixel : Send + Sync , { # [doc = " Constructs a new `ImageBuffer` by repeated application of the supplied function,"] # [doc = " utilizing multi-threading via `rayon`."] # [doc = ""] # [doc = " The arguments to the function are the pixel's x and y coordinates."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics when the resulting image is larger than the maximum size of a vector."] pub fn from_par_fn < F > (width : u32 , height : u32 , f : F) -> ImageBuffer < P , Vec < P :: Subpixel > > where F : Fn (u32 , u32) -> P + Send + Sync , { let mut buf = ImageBuffer :: new (width , height) ; buf . par_enumerate_pixels_mut () . for_each (| (x , y , p) | { * p = f (x , y) ; }) ; buf } }
};
}
