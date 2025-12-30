// Generated macro for DBusRemoveWatchFunction (type)
macro_rules! DepcrateDBusRemoveWatchFunction {
() => {
// Module: crate
// Provides: {"DBusRemoveWatchFunction"}
// Dependencies: {}
pub type DBusRemoveWatchFunction = Option < extern "C" fn (watch : * mut DBusWatch , user_data : * mut c_void) > ;
};
}
