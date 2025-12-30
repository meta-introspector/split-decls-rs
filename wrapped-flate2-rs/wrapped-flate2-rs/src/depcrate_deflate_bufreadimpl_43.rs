// Generated macro for impl_43 (impl)
macro_rules! Depcrate_deflate_bufreadimpl_43 {
() => {
// Module: crate::deflate::bufread
// Provides: {"impl_43"}
// Dependencies: {}
impl < R > DeflateEncoder < R > { # [doc = " Resets the state of this encoder entirely, swapping out the input"] # [doc = " stream for another."] # [doc = ""] # [doc = " This function will reset the internal state of this encoder and replace"] # [doc = " the input stream with the one provided, returning the previous input"] # [doc = " stream. Future data read from this encoder will be the compressed"] # [doc = " version of `r`'s data."] pub fn reset (& mut self , r : R) -> R { reset_encoder_data (self) ; mem :: replace (& mut self . obj , r) } # [doc = " Acquires a reference to the underlying reader"] pub fn get_ref (& self) -> & R { & self . obj } # [doc = " Acquires a mutable reference to the underlying stream"] # [doc = ""] # [doc = " Note that mutation of the stream may result in surprising results if"] # [doc = " this encoder is continued to be used."] pub fn get_mut (& mut self) -> & mut R { & mut self . obj } # [doc = " Consumes this encoder, returning the underlying reader."] pub fn into_inner (self) -> R { self . obj } # [doc = " Returns the number of bytes that have been read into this compressor."] # [doc = ""] # [doc = " Note that not all bytes read from the underlying object may be accounted"] # [doc = " for, there may still be some active buffering."] pub fn total_in (& self) -> u64 { self . data . total_in () } # [doc = " Returns the number of bytes that the compressor has produced."] # [doc = ""] # [doc = " Note that not all bytes may have been read yet, some may still be"] # [doc = " buffered."] pub fn total_out (& self) -> u64 { self . data . total_out () } }
};
}
