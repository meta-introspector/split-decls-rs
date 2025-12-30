// Generated macro for impl_195 (impl)
macro_rules! Depcrate_flatimpl_195 {
() => {
// Module: crate::flat
// Provides: {"impl_195"}
// Dependencies: {}
impl < 'buf , Subpixel > FlatSamples < & 'buf [Subpixel] > { # [doc = " Create a monocolor image from a single pixel."] # [doc = ""] # [doc = " This can be used as a very cheap source of a `GenericImageView` with an arbitrary number of"] # [doc = " pixels of a single color, without any dynamic allocation."] # [doc = ""] # [doc = " ## Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # fn paint_something<T>(_: T) {}"] # [doc = " use image::{flat::FlatSamples, GenericImage, RgbImage, Rgb};"] # [doc = ""] # [doc = " let background = Rgb([20, 20, 20]);"] # [doc = " let bg = FlatSamples::with_monocolor(&background, 200, 200);"] # [doc = ""] # [doc = " let mut image = RgbImage::new(200, 200);"] # [doc = " paint_something(&mut image);"] # [doc = ""] # [doc = " // Reset the canvas"] # [doc = " image.copy_from(&bg.as_view().unwrap(), 0, 0);"] # [doc = " ```"] pub fn with_monocolor < P > (pixel : & 'buf P , width : u32 , height : u32) -> Self where P : Pixel < Subpixel = Subpixel > , Subpixel : crate :: Primitive , { FlatSamples { samples : pixel . channels () , layout : SampleLayout { channels : P :: CHANNEL_COUNT , channel_stride : 1 , width , width_stride : 0 , height , height_stride : 0 , } , color_hint : None , } } }
};
}
