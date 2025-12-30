// Generated macro for CFRunLoopObserverCallBack (type)
macro_rules! Depcrate_runloopCFRunLoopObserverCallBack {
() => {
// Module: crate::runloop
// Provides: {"CFRunLoopObserverCallBack"}
// Dependencies: {}
pub type CFRunLoopObserverCallBack = extern "C" fn (observer : CFRunLoopObserverRef , activity : CFRunLoopActivity , info : * mut c_void) ;
};
}
