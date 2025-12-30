// Generated macro for impl_695 (impl)
macro_rules! Depcrate_messageimpl_695 {
() => {
// Module: crate::message
// Provides: {"impl_695"}
// Dependencies: {}
impl MessageTrailersStrs { # [doc = " Create a borrowed iterator."] pub fn iter (& self) -> MessageTrailersStrsIterator < '_ > { MessageTrailersStrsIterator (self . 0 . iter ()) } # [doc = " The number of trailer key–value pairs."] pub fn len (& self) -> usize { self . 0 . len () } # [doc = " Convert to the “bytes” variant."] pub fn to_bytes (self) -> MessageTrailersBytes { MessageTrailersBytes (self . 0) } }
};
}
