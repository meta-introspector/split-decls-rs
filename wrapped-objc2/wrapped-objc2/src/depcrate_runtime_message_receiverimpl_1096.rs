// Generated macro for impl_1096 (impl)
macro_rules! Depcrate_runtime_message_receiverimpl_1096 {
() => {
// Module: crate::runtime::message_receiver
// Provides: {"impl_1096"}
// Dependencies: {}
unsafe impl < T : ? Sized + Message > MessageReceiver for & T { type __Inner = T ; # [inline] fn __as_raw_receiver (self) -> * mut AnyObject { let ptr : * const T = self ; (ptr as * mut T) . cast () } }
};
}
