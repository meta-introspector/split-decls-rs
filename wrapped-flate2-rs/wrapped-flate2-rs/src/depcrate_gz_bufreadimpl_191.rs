// Generated macro for impl_191 (impl)
macro_rules! Depcrate_gz_bufreadimpl_191 {
() => {
// Module: crate::gz::bufread
// Provides: {"impl_191"}
// Dependencies: {}
impl < R > MultiGzDecoder < R > { # [doc = " Returns the current header associated with this stream, if it's valid"] pub fn header (& self) -> Option < & GzHeader > { self . 0 . header () } # [doc = " Acquires a reference to the underlying reader."] pub fn get_ref (& self) -> & R { self . 0 . get_ref () } # [doc = " Acquires a mutable reference to the underlying stream."] # [doc = ""] # [doc = " Note that mutation of the stream may result in surprising results if"] # [doc = " this decoder is continued to be used."] pub fn get_mut (& mut self) -> & mut R { self . 0 . get_mut () } # [doc = " Consumes this decoder, returning the underlying reader."] pub fn into_inner (self) -> R { self . 0 . into_inner () } }
};
}
