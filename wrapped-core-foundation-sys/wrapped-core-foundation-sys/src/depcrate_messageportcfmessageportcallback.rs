// Generated macro for CFMessagePortCallBack (type)
macro_rules! Depcrate_messageportCFMessagePortCallBack {
() => {
// Module: crate::messageport
// Provides: {"CFMessagePortCallBack"}
// Dependencies: {}
pub type CFMessagePortCallBack = Option < unsafe extern "C" fn (local : CFMessagePortRef , msgid : i32 , data : CFDataRef , info : * mut c_void ,) -> CFDataRef , > ;
};
}
