// Generated macro for OrderedQueue (struct)
macro_rules! Depcrate_core_ordered_queueOrderedQueue {
() => {
// Module: crate::core::ordered_queue
// Provides: {"OrderedQueue"}
// Dependencies: {}
pub (crate) struct OrderedQueue < T > where T : Send , { sender : Sender < Ordered < T > > , pending_count : Arc < AtomicUsize > , stop : Arc < AtomicBool > , }
};
}
