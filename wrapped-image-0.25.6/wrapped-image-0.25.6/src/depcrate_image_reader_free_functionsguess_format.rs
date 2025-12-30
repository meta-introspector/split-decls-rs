// Generated macro for guess_format (function)
macro_rules! Depcrate_image_reader_free_functionsguess_format {
() => {
// Module: crate::image_reader::free_functions
// Provides: {"guess_format"}
// Dependencies: {}
# [doc = " Guess image format from memory block"] # [doc = ""] # [doc = " Makes an educated guess about the image format based on the Magic Bytes at the beginning."] # [doc = " TGA is not supported by this function."] # [doc = " This is not to be trusted on the validity of the whole memory block"] pub fn guess_format (buffer : & [u8]) -> ImageResult < ImageFormat > { match guess_format_impl (buffer) { Some (format) => Ok (format) , None => Err (ImageError :: Unsupported (ImageFormatHint :: Unknown . into ())) , } }
};
}
