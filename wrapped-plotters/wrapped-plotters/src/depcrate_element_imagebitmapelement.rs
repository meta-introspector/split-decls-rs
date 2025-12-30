// Generated macro for BitMapElement (struct)
macro_rules! Depcrate_element_imageBitMapElement {
() => {
// Module: crate::element::image
// Provides: {"BitMapElement"}
// Dependencies: {}
# [doc = " The element that contains a bitmap on it"] pub struct BitMapElement < 'a , Coord , P : PixelFormat = RGBPixel > { image : Buffer < 'a > , size : (u32 , u32) , pos : Coord , phantom : PhantomData < P > , }
};
}
