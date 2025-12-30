// Generated macro for load (function)
macro_rules! Depcrate_image_reader_free_functionsload {
() => {
// Module: crate::image_reader::free_functions
// Provides: {"load"}
// Dependencies: {}
# [doc = " Create a new image from a Reader."] # [doc = ""] # [doc = " Assumes the reader is already buffered. For optimal performance,"] # [doc = " consider wrapping the reader with a `BufReader::new()`."] # [doc = ""] # [doc = " Try [`ImageReader`] for more advanced uses."] pub fn load < R : BufRead + Seek > (r : R , format : ImageFormat) -> ImageResult < DynamicImage > { let mut reader = ImageReader :: new (r) ; reader . set_format (format) ; reader . decode () }
};
}
