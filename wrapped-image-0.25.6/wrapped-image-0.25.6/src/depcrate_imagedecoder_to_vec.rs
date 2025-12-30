// Generated macro for decoder_to_vec (function)
macro_rules! Depcrate_imagedecoder_to_vec {
() => {
// Module: crate::image
// Provides: {"decoder_to_vec"}
// Dependencies: {}
# [doc = " Reads all of the bytes of a decoder into a Vec<T>. No particular alignment"] # [doc = " of the output buffer is guaranteed."] # [doc = ""] # [doc = " Panics if there isn't enough memory to decode the image."] pub (crate) fn decoder_to_vec < T > (decoder : impl ImageDecoder) -> ImageResult < Vec < T > > where T : crate :: traits :: Primitive + bytemuck :: Pod , { let total_bytes = usize :: try_from (decoder . total_bytes ()) ; if total_bytes . is_err () || total_bytes . unwrap () > isize :: MAX as usize { return Err (ImageError :: Limits (LimitError :: from_kind (LimitErrorKind :: InsufficientMemory ,))) ; } let mut buf = vec ! [num_traits :: Zero :: zero () ; total_bytes . unwrap () / size_of ::< T > ()] ; decoder . read_image (bytemuck :: cast_slice_mut (buf . as_mut_slice ())) ? ; Ok (buf) }
};
}
