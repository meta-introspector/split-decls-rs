// Generated macro for DBusAddWatchFunction (type)
macro_rules! DepcrateDBusAddWatchFunction {
() => {
// Module: crate
// Provides: {"DBusAddWatchFunction"}
// Dependencies: {}
pub type DBusAddWatchFunction = Option < extern "C" fn (watch : * mut DBusWatch , user_data : * mut c_void) -> u32 > ;
};
}
