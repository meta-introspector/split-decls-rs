// Generated macro for impl_708 (impl)
macro_rules! Depcrate_messageimpl_708 {
() => {
// Module: crate::message
// Provides: {"impl_708"}
// Dependencies: {}
impl DoubleEndedIterator for MessageTrailersStrsIterator < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { self . 0 . range . next_back () . map (| index | to_str_tuple (& self . 0 . trailers , index)) } }
};
}
