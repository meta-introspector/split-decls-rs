// Generated macro for DBusTimeoutToggledFunction (type)
macro_rules! DepcrateDBusTimeoutToggledFunction {
() => {
// Module: crate
// Provides: {"DBusTimeoutToggledFunction"}
// Dependencies: {}
pub type DBusTimeoutToggledFunction = Option < extern "C" fn (timeout : * mut DBusTimeout , user_data : * mut c_void) > ;
};
}
