// Generated macro for DBusWatchToggledFunction (type)
macro_rules! DepcrateDBusWatchToggledFunction {
() => {
// Module: crate
// Provides: {"DBusWatchToggledFunction"}
// Dependencies: {}
pub type DBusWatchToggledFunction = Option < extern "C" fn (watch : * mut DBusWatch , user_data : * mut c_void) > ;
};
}
