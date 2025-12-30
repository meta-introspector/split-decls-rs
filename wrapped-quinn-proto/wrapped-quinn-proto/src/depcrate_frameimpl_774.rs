// Generated macro for impl_774 (impl)
macro_rules! Depcrate_frameimpl_774 {
() => {
// Module: crate::frame
// Provides: {"impl_774"}
// Dependencies: {}
impl AckFrequency { pub (crate) fn encode < W : BufMut > (& self , buf : & mut W) { buf . write (FrameType :: ACK_FREQUENCY) ; buf . write (self . sequence) ; buf . write (self . ack_eliciting_threshold) ; buf . write (self . request_max_ack_delay) ; buf . write (self . reordering_threshold) ; } }
};
}
