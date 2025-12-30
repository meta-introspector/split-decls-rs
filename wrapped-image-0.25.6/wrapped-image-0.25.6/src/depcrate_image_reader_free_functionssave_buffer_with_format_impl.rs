// Generated macro for save_buffer_with_format_impl (function)
macro_rules! Depcrate_image_reader_free_functionssave_buffer_with_format_impl {
() => {
// Module: crate::image_reader::free_functions
// Provides: {"save_buffer_with_format_impl"}
// Dependencies: {}
# [allow (unused_variables)] pub (crate) fn save_buffer_with_format_impl (path : & Path , buf : & [u8] , width : u32 , height : u32 , color : ExtendedColorType , format : ImageFormat ,) -> ImageResult < () > { let buffered_file_write = & mut BufWriter :: new (File :: create (path) ?) ; write_buffer_impl (buffered_file_write , buf , width , height , color , format) }
};
}
