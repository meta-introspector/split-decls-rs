// Generated macro for DBusPendingCallNotifyFunction (type)
macro_rules! DepcrateDBusPendingCallNotifyFunction {
() => {
// Module: crate
// Provides: {"DBusPendingCallNotifyFunction"}
// Dependencies: {}
pub type DBusPendingCallNotifyFunction = Option < extern "C" fn (pending : * mut DBusPendingCall , user_data : * mut c_void) > ;
};
}
