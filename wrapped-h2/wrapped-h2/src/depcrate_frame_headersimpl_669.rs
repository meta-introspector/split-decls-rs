// Generated macro for impl_669 (impl)
macro_rules! Depcrate_frame_headersimpl_669 {
() => {
// Module: crate::frame::headers
// Provides: {"impl_669"}
// Dependencies: {}
impl PushPromiseFlag { pub fn empty () -> PushPromiseFlag { PushPromiseFlag (0) } pub fn load (bits : u8) -> PushPromiseFlag { PushPromiseFlag (bits & ALL) } pub fn is_end_headers (& self) -> bool { self . 0 & END_HEADERS == END_HEADERS } pub fn set_end_headers (& mut self) { self . 0 |= END_HEADERS ; } pub fn is_padded (& self) -> bool { self . 0 & PADDED == PADDED } }
};
}
