// Generated macro for impl_661 (impl)
macro_rules! Depcrate_frame_headersimpl_661 {
() => {
// Module: crate::frame::headers
// Provides: {"impl_661"}
// Dependencies: {}
impl Continuation { fn head (& self) -> Head { Head :: new (Kind :: Continuation , END_HEADERS , self . stream_id) } pub fn encode (self , dst : & mut EncodeBuf < '_ >) -> Option < Continuation > { let head = self . head () ; self . header_block . encode (& head , dst , | _ | { }) } }
};
}
