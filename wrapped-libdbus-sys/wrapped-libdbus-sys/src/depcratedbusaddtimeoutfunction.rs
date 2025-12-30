// Generated macro for DBusAddTimeoutFunction (type)
macro_rules! DepcrateDBusAddTimeoutFunction {
() => {
// Module: crate
// Provides: {"DBusAddTimeoutFunction"}
// Dependencies: {}
pub type DBusAddTimeoutFunction = Option < extern "C" fn (timeout : * mut DBusTimeout , user_data : * mut c_void) -> u32 > ;
};
}
