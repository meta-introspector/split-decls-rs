// Generated macro for save_buffer_with_format (function)
macro_rules! Depcrate_dynimagesave_buffer_with_format {
() => {
// Module: crate::dynimage
// Provides: {"save_buffer_with_format"}
// Dependencies: {}
# [doc = " Saves the supplied buffer to a file at the path specified"] # [doc = " in the specified format."] # [doc = ""] # [doc = " The buffer is assumed to have the correct format according"] # [doc = " to the specified color type."] # [doc = " This will lead to corrupted files if the buffer contains"] # [doc = " malformed data. Currently only jpeg, png, ico, bmp, exr and"] # [doc = " tiff files are supported."] pub fn save_buffer_with_format (path : impl AsRef < Path > , buf : & [u8] , width : u32 , height : u32 , color : impl Into < ExtendedColorType > , format : ImageFormat ,) -> ImageResult < () > { free_functions :: save_buffer_with_format_impl (path . as_ref () , buf , width , height , color . into () , format ,) }
};
}
