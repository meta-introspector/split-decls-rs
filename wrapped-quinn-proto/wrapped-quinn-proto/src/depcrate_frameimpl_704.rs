// Generated macro for impl_704 (impl)
macro_rules! Depcrate_frameimpl_704 {
() => {
// Module: crate::frame
// Provides: {"impl_704"}
// Dependencies: {}
impl FrameType { fn stream (self) -> Option < StreamInfo > { if STREAM_TYS . contains (& self . 0) { Some (StreamInfo (self . 0 as u8)) } else { None } } fn datagram (self) -> Option < DatagramInfo > { if DATAGRAM_TYS . contains (& self . 0) { Some (DatagramInfo (self . 0 as u8)) } else { None } } }
};
}
