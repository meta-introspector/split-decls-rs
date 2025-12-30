// Generated macro for DBusDispatchStatusFunction (type)
macro_rules! DepcrateDBusDispatchStatusFunction {
() => {
// Module: crate
// Provides: {"DBusDispatchStatusFunction"}
// Dependencies: {}
pub type DBusDispatchStatusFunction = Option < extern "C" fn (conn : * mut DBusConnection , new_status : DBusDispatchStatus , user_data : * mut c_void) > ;
};
}
