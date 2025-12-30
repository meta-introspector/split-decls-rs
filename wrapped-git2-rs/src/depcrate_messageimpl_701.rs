// Generated macro for impl_701 (impl)
macro_rules! Depcrate_messageimpl_701 {
() => {
// Module: crate::message
// Provides: {"impl_701"}
// Dependencies: {}
impl Binding for MessageTrailers { type Raw = * mut raw :: git_message_trailer_array ; unsafe fn from_raw (raw : * mut raw :: git_message_trailer_array) -> MessageTrailers { MessageTrailers { raw : * raw } } fn raw (& self) -> * mut raw :: git_message_trailer_array { & self . raw as * const _ as * mut _ } }
};
}
