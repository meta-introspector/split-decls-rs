// Generated macro for BorrowRead (trait)
macro_rules! Depcrate_decode_value_refBorrowRead {
() => {
// Module: crate::decode::value_ref
// Provides: {"BorrowRead"}
// Dependencies: {}
# [doc = " A `BorrowRead` is a type of Reader which has an internal buffer."] # [doc = ""] # [doc = " This magic trait acts like a standard `BufRead` but unlike the standard this has an explicit"] # [doc = " internal buffer lifetime, which allows to borrow from underlying buffer while consuming bytes."] pub trait BorrowRead < 'a > : Read { # [doc = " Returns the buffer contents."] # [doc = ""] # [doc = " This function is a lower-level call. It needs to be paired with the consume method to"] # [doc = " function properly. When calling this method, none of the contents will be \"read\" in the"] # [doc = " sense that later calling read may return the same contents. As such, consume must be called"] # [doc = " with the number of bytes that are consumed from this buffer to ensure that the bytes are"] # [doc = " never returned twice."] # [doc = ""] # [doc = " An empty buffer returned indicates that the stream has reached EOF."] fn fill_buf (& self) -> & 'a [u8] ; # [doc = " Tells this buffer that len bytes have been consumed from the buffer, so they should no"] # [doc = " longer be returned in calls to read."] fn consume (& mut self , len : usize) ; }
};
}
