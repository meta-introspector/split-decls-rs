// Generated macro for CFRunLoopSourceContext (struct)
macro_rules! Depcrate_runloopCFRunLoopSourceContext {
() => {
// Module: crate::runloop
// Provides: {"CFRunLoopSourceContext"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Clone , Copy)] pub struct CFRunLoopSourceContext { pub version : CFIndex , pub info : * mut c_void , pub retain : Option < extern "C" fn (info : * const c_void) -> * const c_void > , pub release : Option < extern "C" fn (info : * const c_void) > , pub copyDescription : Option < extern "C" fn (info : * const c_void) -> CFStringRef > , pub equal : Option < extern "C" fn (info1 : * const c_void , info2 : * const c_void) -> Boolean > , pub hash : Option < extern "C" fn (info : * const c_void) -> CFHashCode > , pub schedule : Option < extern "C" fn (info : * const c_void , rl : CFRunLoopRef , mode : CFStringRef) > , pub cancel : Option < extern "C" fn (info : * const c_void , rl : CFRunLoopRef , mode : CFStringRef) > , pub perform : extern "C" fn (info : * const c_void) , }
};
}
