// Generated macro for DBusHandleMessageFunction (type)
macro_rules! DepcrateDBusHandleMessageFunction {
() => {
// Module: crate
// Provides: {"DBusHandleMessageFunction"}
// Dependencies: {}
pub type DBusHandleMessageFunction = Option < extern "C" fn (conn : * mut DBusConnection , msg : * mut DBusMessage , user_data : * mut c_void) -> DBusHandlerResult > ;
};
}
