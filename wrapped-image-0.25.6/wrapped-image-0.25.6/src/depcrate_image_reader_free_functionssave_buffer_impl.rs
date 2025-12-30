// Generated macro for save_buffer_impl (function)
macro_rules! Depcrate_image_reader_free_functionssave_buffer_impl {
() => {
// Module: crate::image_reader::free_functions
// Provides: {"save_buffer_impl"}
// Dependencies: {}
# [allow (unused_variables)] pub (crate) fn save_buffer_impl (path : & Path , buf : & [u8] , width : u32 , height : u32 , color : ExtendedColorType ,) -> ImageResult < () > { let format = ImageFormat :: from_path (path) ? ; save_buffer_with_format_impl (path , buf , width , height , color , format) }
};
}
