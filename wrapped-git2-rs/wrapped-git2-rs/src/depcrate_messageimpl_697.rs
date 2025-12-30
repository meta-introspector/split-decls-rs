// Generated macro for impl_697 (impl)
macro_rules! Depcrate_messageimpl_697 {
() => {
// Module: crate::message
// Provides: {"impl_697"}
// Dependencies: {}
impl MessageTrailersBytes { # [doc = " Create a borrowed iterator."] pub fn iter (& self) -> MessageTrailersBytesIterator < '_ > { MessageTrailersBytesIterator (self . 0 . iter ()) } # [doc = " The number of trailer key–value pairs."] pub fn len (& self) -> usize { self . 0 . len () } }
};
}
