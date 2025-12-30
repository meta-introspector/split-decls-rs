// Generated macro for impl_180 (impl)
macro_rules! Depcrate_queueimpl_180 {
() => {
// Module: crate::queue
// Provides: {"impl_180"}
// Dependencies: {}
impl DispatchQueueAttr { # [doc = " A dispatch queue that executes blocks serially in FIFO order."] # [doc (alias = "DISPATCH_QUEUE_SERIAL")] pub const SERIAL : Option < & Self > = None ; # [doc = " A dispatch queue that executes blocks concurrently."] pub fn concurrent () -> Option < & 'static Self > { unsafe { Some (& _dispatch_queue_attr_concurrent) } } }
};
}
