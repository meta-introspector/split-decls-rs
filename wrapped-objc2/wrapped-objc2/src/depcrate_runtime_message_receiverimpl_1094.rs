// Generated macro for impl_1094 (impl)
macro_rules! Depcrate_runtime_message_receiverimpl_1094 {
() => {
// Module: crate::runtime::message_receiver
// Provides: {"impl_1094"}
// Dependencies: {}
unsafe impl < T : ? Sized + Message > MessageReceiver for NonNull < T > { type __Inner = T ; # [inline] fn __as_raw_receiver (self) -> * mut AnyObject { self . as_ptr () . cast () } }
};
}
