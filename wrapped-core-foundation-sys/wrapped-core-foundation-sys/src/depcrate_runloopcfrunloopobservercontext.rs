// Generated macro for CFRunLoopObserverContext (struct)
macro_rules! Depcrate_runloopCFRunLoopObserverContext {
() => {
// Module: crate::runloop
// Provides: {"CFRunLoopObserverContext"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Clone , Copy)] pub struct CFRunLoopObserverContext { pub version : CFIndex , pub info : * mut c_void , pub retain : Option < extern "C" fn (info : * const c_void) -> * const c_void > , pub release : Option < extern "C" fn (info : * const c_void) > , pub copyDescription : Option < extern "C" fn (info : * const c_void) -> CFStringRef > , }
};
}
