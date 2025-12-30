// Generated macro for save_buffer (function)
macro_rules! Depcrate_dynimagesave_buffer {
() => {
// Module: crate::dynimage
// Provides: {"save_buffer"}
// Dependencies: {}
# [doc = " Saves the supplied buffer to a file at the path specified."] # [doc = ""] # [doc = " The image format is derived from the file extension. The buffer is assumed to have"] # [doc = " the correct format according to the specified color type."] # [doc = ""] # [doc = " This will lead to corrupted files if the buffer contains malformed data. Currently only"] # [doc = " jpeg, png, ico, pnm, bmp, exr and tiff files are supported."] pub fn save_buffer (path : impl AsRef < Path > , buf : & [u8] , width : u32 , height : u32 , color : impl Into < ExtendedColorType > ,) -> ImageResult < () > { free_functions :: save_buffer_impl (path . as_ref () , buf , width , height , color . into ()) }
};
}
