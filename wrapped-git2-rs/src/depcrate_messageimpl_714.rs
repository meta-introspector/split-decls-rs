// Generated macro for impl_714 (impl)
macro_rules! Depcrate_messageimpl_714 {
() => {
// Module: crate::message
// Provides: {"impl_714"}
// Dependencies: {}
impl DoubleEndedIterator for MessageTrailersBytesIterator < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { self . 0 . range . next_back () . map (| index | to_bytes_tuple (& self . 0 . trailers , index)) } }
};
}
