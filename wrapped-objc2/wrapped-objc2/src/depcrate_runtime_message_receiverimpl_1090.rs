// Generated macro for impl_1090 (impl)
macro_rules! Depcrate_runtime_message_receiverimpl_1090 {
() => {
// Module: crate::runtime::message_receiver
// Provides: {"impl_1090"}
// Dependencies: {}
unsafe impl < T : ? Sized + Message > MessageReceiver for * const T { type __Inner = T ; # [inline] fn __as_raw_receiver (self) -> * mut AnyObject { (self as * mut T) . cast () } }
};
}
