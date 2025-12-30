// Generated macro for buffer (module)
macro_rules! Depcratebuffer {
() => {
// Module: crate
// Provides: {"buffer"}
// Dependencies: {}
# [doc = " Iterators and other auxiliary structure for the `ImageBuffer` type."] pub mod buffer { pub use crate :: buffer_ :: { ConvertBuffer , EnumeratePixels , EnumeratePixelsMut , EnumerateRows , EnumerateRowsMut , Pixels , PixelsMut , Rows , RowsMut , } ; # [cfg (feature = "rayon")] pub use crate :: buffer_par :: * ; }
};
}
