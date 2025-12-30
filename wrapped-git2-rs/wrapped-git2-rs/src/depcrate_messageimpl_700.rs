// Generated macro for impl_700 (impl)
macro_rules! Depcrate_messageimpl_700 {
() => {
// Module: crate::message
// Provides: {"impl_700"}
// Dependencies: {}
impl Drop for MessageTrailers { fn drop (& mut self) { unsafe { raw :: git_message_trailer_array_free (& mut self . raw) ; } } }
};
}
