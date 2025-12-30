// Generated macro for RTL8139Error (enum)
macro_rules! Depcrate_drivers_net_rtl8139RTL8139Error {
() => {
// Module: crate::drivers::net::rtl8139
// Provides: {"RTL8139Error"}
// Dependencies: {}
# [derive (Error , Debug)] pub enum RTL8139Error { # [error ("initialization failed")] InitFailed , # [error ("reset failed")] ResetFailed , # [error ("unknown RTL8139 error")] Unknown , }
};
}
