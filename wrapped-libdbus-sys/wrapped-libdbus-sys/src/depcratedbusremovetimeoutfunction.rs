// Generated macro for DBusRemoveTimeoutFunction (type)
macro_rules! DepcrateDBusRemoveTimeoutFunction {
() => {
// Module: crate
// Provides: {"DBusRemoveTimeoutFunction"}
// Dependencies: {}
pub type DBusRemoveTimeoutFunction = Option < extern "C" fn (timeout : * mut DBusTimeout , user_data : * mut c_void) > ;
};
}
