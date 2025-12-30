// Generated macro for CFStreamClientContext (struct)
macro_rules! Depcrate_streamCFStreamClientContext {
() => {
// Module: crate::stream
// Provides: {"CFStreamClientContext"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Clone , Copy)] pub struct CFStreamClientContext { pub version : CFIndex , pub info : * mut c_void , pub retain : extern "C" fn (info : * const c_void) -> * const c_void , pub release : extern "C" fn (info : * const c_void) , pub copyDescription : extern "C" fn (info : * const c_void) -> CFStringRef , }
};
}
