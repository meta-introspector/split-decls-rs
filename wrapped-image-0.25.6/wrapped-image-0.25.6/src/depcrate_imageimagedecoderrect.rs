// Generated macro for ImageDecoderRect (trait)
macro_rules! Depcrate_imageImageDecoderRect {
() => {
// Module: crate::image
// Provides: {"ImageDecoderRect"}
// Dependencies: {}
# [doc = " Specialized image decoding not be supported by all formats"] pub trait ImageDecoderRect : ImageDecoder { # [doc = " Decode a rectangular section of the image."] # [doc = ""] # [doc = " This function takes a slice of bytes and writes the pixel data of the image into it."] # [doc = " The rectangle is specified by the x and y coordinates of the top left corner, the width"] # [doc = " and height of the rectangle, and the row pitch of the buffer. The row pitch is the number"] # [doc = " of bytes between the start of one row and the start of the next row. The row pitch must be"] # [doc = " at least as large as the width of the rectangle in bytes."] fn read_rect (& mut self , x : u32 , y : u32 , width : u32 , height : u32 , buf : & mut [u8] , row_pitch : usize ,) -> ImageResult < () > ; }
};
}
