// Generated macro for impl_209 (impl)
macro_rules! Depcrate_gz_readimpl_209 {
() => {
// Module: crate::gz::read
// Provides: {"impl_209"}
// Dependencies: {}
impl < R > GzDecoder < R > { # [doc = " Returns the header associated with this stream, if it was valid."] pub fn header (& self) -> Option < & GzHeader > { self . inner . header () } # [doc = " Acquires a reference to the underlying reader."] # [doc = ""] # [doc = " Note that the decoder may have read past the end of the gzip data."] # [doc = " To prevent this use [`bufread::GzDecoder`] instead."] pub fn get_ref (& self) -> & R { self . inner . get_ref () . get_ref () } # [doc = " Acquires a mutable reference to the underlying stream."] # [doc = ""] # [doc = " Note that mutation of the stream may result in surprising results if"] # [doc = " this decoder continues to be used."] # [doc = ""] # [doc = " Note that the decoder may have read past the end of the gzip data."] # [doc = " To prevent this use [`bufread::GzDecoder`] instead."] pub fn get_mut (& mut self) -> & mut R { self . inner . get_mut () . get_mut () } # [doc = " Consumes this decoder, returning the underlying reader."] # [doc = ""] # [doc = " Note that the decoder may have read past the end of the gzip data."] # [doc = " Subsequent reads will skip those bytes. To prevent this use"] # [doc = " [`bufread::GzDecoder`] instead."] pub fn into_inner (self) -> R { self . inner . into_inner () . into_inner () } }
};
}
