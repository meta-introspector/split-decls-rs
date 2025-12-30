// Generated macro for private (module)
macro_rules! Depcrate_traitsprivate {
() => {
// Module: crate::traits
// Provides: {"private"}
// Dependencies: {}
# [doc = " Prevents down-stream users from implementing the `Primitive` trait"] mod private { use crate :: color :: * ; pub trait SealedPixelWithColorType { } impl SealedPixelWithColorType for Rgb < u8 > { } impl SealedPixelWithColorType for Rgb < u16 > { } impl SealedPixelWithColorType for Rgb < f32 > { } impl SealedPixelWithColorType for Rgba < u8 > { } impl SealedPixelWithColorType for Rgba < u16 > { } impl SealedPixelWithColorType for Rgba < f32 > { } impl SealedPixelWithColorType for Luma < u8 > { } impl SealedPixelWithColorType for LumaA < u8 > { } impl SealedPixelWithColorType for Luma < u16 > { } impl SealedPixelWithColorType for LumaA < u16 > { } }
};
}
