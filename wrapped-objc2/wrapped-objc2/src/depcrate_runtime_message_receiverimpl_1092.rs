// Generated macro for impl_1092 (impl)
macro_rules! Depcrate_runtime_message_receiverimpl_1092 {
() => {
// Module: crate::runtime::message_receiver
// Provides: {"impl_1092"}
// Dependencies: {}
unsafe impl < T : ? Sized + Message > MessageReceiver for * mut T { type __Inner = T ; # [inline] fn __as_raw_receiver (self) -> * mut AnyObject { self . cast () } }
};
}
