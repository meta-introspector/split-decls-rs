// Generated macro for impl_22 (impl)
macro_rules! Depcrate_messageimpl_22 {
() => {
// Module: crate::message
// Provides: {"impl_22"}
// Dependencies: {}
impl Drop for Message { fn drop (& mut self) { unsafe { ffi :: dbus_message_unref (self . msg) ; } } }
};
}
