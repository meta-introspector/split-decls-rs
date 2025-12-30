// Generated macro for load_from_memory_with_format (function)
macro_rules! Depcrate_dynimageload_from_memory_with_format {
() => {
// Module: crate::dynimage
// Provides: {"load_from_memory_with_format"}
// Dependencies: {}
# [doc = " Create a new image from a byte slice"] # [doc = ""] # [doc = " This is just a simple wrapper that constructs an `std::io::Cursor` around the buffer and then"] # [doc = " calls `load` with that reader."] # [doc = ""] # [doc = " Try [`ImageReader`] for more advanced uses."] # [doc = ""] # [doc = " [`load`]: fn.load.html"] # [inline (always)] pub fn load_from_memory_with_format (buf : & [u8] , format : ImageFormat) -> ImageResult < DynamicImage > { let b = io :: Cursor :: new (buf) ; free_functions :: load (b , format) }
};
}
