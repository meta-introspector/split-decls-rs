// Generated macro for impl_214 (impl)
macro_rules! Depcrate_gz_readimpl_214 {
() => {
// Module: crate::gz::read
// Provides: {"impl_214"}
// Dependencies: {}
impl < R > MultiGzDecoder < R > { # [doc = " Returns the current header associated with this stream, if it's valid."] pub fn header (& self) -> Option < & GzHeader > { self . inner . header () } # [doc = " Acquires a reference to the underlying reader."] pub fn get_ref (& self) -> & R { self . inner . get_ref () . get_ref () } # [doc = " Acquires a mutable reference to the underlying stream."] # [doc = ""] # [doc = " Note that mutation of the stream may result in surprising results if"] # [doc = " this decoder is continued to be used."] pub fn get_mut (& mut self) -> & mut R { self . inner . get_mut () . get_mut () } # [doc = " Consumes this decoder, returning the underlying reader."] pub fn into_inner (self) -> R { self . inner . into_inner () . into_inner () } }
};
}
