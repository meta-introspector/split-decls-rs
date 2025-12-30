// Generated macro for DBusWakeupMainFunction (type)
macro_rules! DepcrateDBusWakeupMainFunction {
() => {
// Module: crate
// Provides: {"DBusWakeupMainFunction"}
// Dependencies: {}
pub type DBusWakeupMainFunction = Option < extern "C" fn (conn : * mut DBusConnection , user_data : * mut c_void) > ;
};
}
