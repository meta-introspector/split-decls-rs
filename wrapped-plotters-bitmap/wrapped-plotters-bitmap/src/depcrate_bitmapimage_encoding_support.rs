// Generated macro for image_encoding_support (module)
macro_rules! Depcrate_bitmapimage_encoding_support {
() => {
// Module: crate::bitmap
// Provides: {"image_encoding_support"}
// Dependencies: {}
# [cfg (all (not (target_arch = "wasm32") , feature = "image"))] mod image_encoding_support { pub (super) use image :: { ImageBuffer , Rgb } ; pub (super) use std :: path :: Path ; pub (super) type BorrowedImage < 'a > = ImageBuffer < Rgb < u8 > , & 'a mut [u8] > ; }
};
}
