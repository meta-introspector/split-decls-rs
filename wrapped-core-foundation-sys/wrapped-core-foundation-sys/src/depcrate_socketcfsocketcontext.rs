// Generated macro for CFSocketContext (struct)
macro_rules! Depcrate_socketCFSocketContext {
() => {
// Module: crate::socket
// Provides: {"CFSocketContext"}
// Dependencies: {}
# [derive (Debug , Copy , Clone)] # [repr (C)] pub struct CFSocketContext { pub version : CFIndex , pub info : * mut c_void , pub retain : extern "C" fn (info : * const c_void) -> * const c_void , pub release : extern "C" fn (info : * const c_void) , pub copyDescription : extern "C" fn (info : * const c_void) -> CFStringRef , }
};
}
