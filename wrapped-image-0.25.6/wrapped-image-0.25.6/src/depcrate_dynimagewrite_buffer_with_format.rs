// Generated macro for write_buffer_with_format (function)
macro_rules! Depcrate_dynimagewrite_buffer_with_format {
() => {
// Module: crate::dynimage
// Provides: {"write_buffer_with_format"}
// Dependencies: {}
# [doc = " Writes the supplied buffer to a writer in the specified format."] # [doc = ""] # [doc = " The buffer is assumed to have the correct format according to the specified color type. This"] # [doc = " will lead to corrupted writers if the buffer contains malformed data."] # [doc = ""] # [doc = " Assumes the writer is buffered. In most cases, you should wrap your writer in a `BufWriter` for"] # [doc = " best performance."] pub fn write_buffer_with_format < W : Write + Seek > (buffered_writer : & mut W , buf : & [u8] , width : u32 , height : u32 , color : impl Into < ExtendedColorType > , format : ImageFormat ,) -> ImageResult < () > { free_functions :: write_buffer_impl (buffered_writer , buf , width , height , color . into () , format) }
};
}
