// Generated macro for guess_format_impl (function)
macro_rules! Depcrate_image_reader_free_functionsguess_format_impl {
() => {
// Module: crate::image_reader::free_functions
// Provides: {"guess_format_impl"}
// Dependencies: {}
pub (crate) fn guess_format_impl (buffer : & [u8]) -> Option < ImageFormat > { for & (signature , mask , format) in & MAGIC_BYTES { if mask . is_empty () { if buffer . starts_with (signature) { return Some (format) ; } } else if buffer . len () >= signature . len () && buffer . iter () . zip (signature . iter ()) . zip (mask . iter () . chain (iter :: repeat (& 0xFF))) . all (| ((& byte , & sig) , & mask) | byte & mask == sig) { return Some (format) ; } } None }
};
}
