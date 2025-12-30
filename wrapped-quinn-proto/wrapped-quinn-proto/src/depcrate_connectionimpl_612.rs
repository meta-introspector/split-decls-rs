// Generated macro for impl_612 (impl)
macro_rules! Depcrate_connectionimpl_612 {
() => {
// Module: crate::connection
// Provides: {"impl_612"}
// Dependencies: {}
impl SentFrames { # [doc = " Returns whether the packet contains only ACKs"] fn is_ack_only (& self , streams : & StreamsState) -> bool { self . largest_acked . is_some () && ! self . non_retransmits && self . stream_frames . is_empty () && self . retransmits . is_empty (streams) } }
};
}
