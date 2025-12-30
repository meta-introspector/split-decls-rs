// Generated macro for load_from_memory (function)
macro_rules! Depcrate_dynimageload_from_memory {
() => {
// Module: crate::dynimage
// Provides: {"load_from_memory"}
// Dependencies: {}
# [doc = " Create a new image from a byte slice"] # [doc = ""] # [doc = " Makes an educated guess about the image format."] # [doc = " TGA is not supported by this function."] # [doc = ""] # [doc = " Try [`ImageReader`] for more advanced uses."] pub fn load_from_memory (buffer : & [u8]) -> ImageResult < DynamicImage > { let format = free_functions :: guess_format (buffer) ? ; load_from_memory_with_format (buffer , format) }
};
}
