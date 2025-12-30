// Generated macro for NSApp (function)
macro_rules! Depcrate_appkitNSApp {
() => {
// Module: crate::appkit
// Provides: {"NSApp"}
// Dependencies: {}
pub unsafe fn NSApp () -> id { msg_send ! [class ! (NSApplication) , sharedApplication] }
};
}
