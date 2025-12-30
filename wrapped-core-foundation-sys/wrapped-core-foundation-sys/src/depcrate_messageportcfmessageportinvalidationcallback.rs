// Generated macro for CFMessagePortInvalidationCallBack (type)
macro_rules! Depcrate_messageportCFMessagePortInvalidationCallBack {
() => {
// Module: crate::messageport
// Provides: {"CFMessagePortInvalidationCallBack"}
// Dependencies: {}
pub type CFMessagePortInvalidationCallBack = Option < unsafe extern "C" fn (ms : CFMessagePortRef , info : * mut c_void) > ;
};
}
