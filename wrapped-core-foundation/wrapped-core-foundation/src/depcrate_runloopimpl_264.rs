// Generated macro for impl_264 (impl)
macro_rules! Depcrate_runloopimpl_264 {
() => {
// Module: crate::runloop
// Provides: {"impl_264"}
// Dependencies: {}
impl CFRunLoopTimer { pub fn new (fireDate : CFAbsoluteTime , interval : CFTimeInterval , flags : CFOptionFlags , order : CFIndex , callout : CFRunLoopTimerCallBack , context : * mut CFRunLoopTimerContext ,) -> CFRunLoopTimer { unsafe { let timer_ref = CFRunLoopTimerCreate (kCFAllocatorDefault , fireDate , interval , flags , order , callout , context ,) ; TCFType :: wrap_under_create_rule (timer_ref) } } }
};
}
