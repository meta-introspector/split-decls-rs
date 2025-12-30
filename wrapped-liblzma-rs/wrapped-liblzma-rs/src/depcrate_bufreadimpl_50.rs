// Generated macro for impl_50 (impl)
macro_rules! Depcrate_bufreadimpl_50 {
() => {
// Module: crate::bufread
// Provides: {"impl_50"}
// Dependencies: {}
impl < R > XzDecoder < R > { # [doc = " Acquires a reference to the underlying stream"] # [inline] pub fn get_ref (& self) -> & R { & self . obj } # [doc = " Acquires a mutable reference to the underlying stream"] # [doc = ""] # [doc = " Note that mutation of the stream may result in surprising results if"] # [doc = " this encoder is continued to be used."] # [inline] pub fn get_mut (& mut self) -> & mut R { & mut self . obj } # [doc = " Consumes this decoder, returning the underlying reader."] # [inline] pub fn into_inner (self) -> R { self . obj } # [doc = " Returns the number of bytes that the decompressor has consumed."] # [doc = ""] # [doc = " Note that this will likely be smaller than what the decompressor"] # [doc = " actually read from the underlying stream due to buffering."] # [inline] pub fn total_in (& self) -> u64 { self . data . total_in () } # [doc = " Returns the number of bytes that the decompressor has produced."] # [inline] pub fn total_out (& self) -> u64 { self . data . total_out () } }
};
}
