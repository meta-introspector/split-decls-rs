// Generated macro for impl_665 (impl)
macro_rules! Depcrate_frame_headersimpl_665 {
() => {
// Module: crate::frame::headers
// Provides: {"impl_665"}
// Dependencies: {}
impl HeadersFlag { pub fn empty () -> HeadersFlag { HeadersFlag (0) } pub fn load (bits : u8) -> HeadersFlag { HeadersFlag (bits & ALL) } pub fn is_end_stream (& self) -> bool { self . 0 & END_STREAM == END_STREAM } pub fn set_end_stream (& mut self) { self . 0 |= END_STREAM ; } pub fn is_end_headers (& self) -> bool { self . 0 & END_HEADERS == END_HEADERS } pub fn set_end_headers (& mut self) { self . 0 |= END_HEADERS ; } pub fn is_padded (& self) -> bool { self . 0 & PADDED == PADDED } pub fn is_priority (& self) -> bool { self . 0 & PRIORITY == PRIORITY } }
};
}
