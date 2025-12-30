// Generated macro for impl_1098 (impl)
macro_rules! Depcrate_runtime_message_receiverimpl_1098 {
() => {
// Module: crate::runtime::message_receiver
// Provides: {"impl_1098"}
// Dependencies: {}
# [doc = " `&mut AnyObject` is allowed as mutable, for easier transition from `objc`,"] # [doc = " even though it's basically always incorrect to hold `&mut AnyObject`."] # [doc = ""] # [doc = " Use `*mut AnyObject` instead if you know for certain you need mutability,"] # [doc = " and cannot make do with interior mutability."] unsafe impl MessageReceiver for & mut AnyObject { type __Inner = AnyObject ; # [inline] fn __as_raw_receiver (self) -> * mut AnyObject { self } }
};
}
