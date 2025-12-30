// Generated macro for CFSocketCallBack (type)
macro_rules! Depcrate_socketCFSocketCallBack {
() => {
// Module: crate::socket
// Provides: {"CFSocketCallBack"}
// Dependencies: {}
pub type CFSocketCallBack = extern "C" fn (s : CFSocketRef , _type : CFSocketCallBackType , address : CFDataRef , cdata : * const c_void , info : * mut c_void ,) ;
};
}
