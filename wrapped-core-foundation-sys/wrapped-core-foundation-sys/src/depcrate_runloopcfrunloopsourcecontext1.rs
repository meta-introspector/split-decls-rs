// Generated macro for CFRunLoopSourceContext1 (struct)
macro_rules! Depcrate_runloopCFRunLoopSourceContext1 {
() => {
// Module: crate::runloop
// Provides: {"CFRunLoopSourceContext1"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Clone , Copy)] pub struct CFRunLoopSourceContext1 { pub version : CFIndex , pub info : * mut c_void , pub retain : Option < extern "C" fn (info : * const c_void) -> * const c_void > , pub release : Option < extern "C" fn (info : * const c_void) > , pub copyDescription : Option < extern "C" fn (info : * const c_void) -> CFStringRef > , pub equal : Option < extern "C" fn (info1 : * const c_void , info2 : * const c_void) -> Boolean > , pub hash : Option < extern "C" fn (info : * const c_void) -> CFHashCode > , # [cfg (any (target_os = "macos" , target_os = "ios"))] pub getPort : extern "C" fn (info : * mut c_void) -> mach_port_t , # [cfg (any (target_os = "macos" , target_os = "ios"))] pub perform : extern "C" fn (msg : * mut c_void , size : CFIndex , allocator : CFAllocatorRef , info : * mut c_void ,) -> * mut c_void , # [cfg (not (any (target_os = "macos" , target_os = "ios")))] pub getPort : extern "C" fn (info : * mut c_void) -> * mut c_void , # [cfg (not (any (target_os = "macos" , target_os = "ios")))] pub perform : extern "C" fn (info : * mut c_void) -> * mut c_void , }
};
}
