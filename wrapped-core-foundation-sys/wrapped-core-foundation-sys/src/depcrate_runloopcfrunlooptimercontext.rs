// Generated macro for CFRunLoopTimerContext (struct)
macro_rules! Depcrate_runloopCFRunLoopTimerContext {
() => {
// Module: crate::runloop
// Provides: {"CFRunLoopTimerContext"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug)] pub struct CFRunLoopTimerContext { pub version : CFIndex , pub info : * mut c_void , pub retain : Option < extern "C" fn (info : * const c_void) -> * const c_void > , pub release : Option < extern "C" fn (info : * const c_void) > , pub copyDescription : Option < extern "C" fn (info : * const c_void) -> CFStringRef > , }
};
}
