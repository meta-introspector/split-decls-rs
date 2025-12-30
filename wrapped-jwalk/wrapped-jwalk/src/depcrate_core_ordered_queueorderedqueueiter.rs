// Generated macro for OrderedQueueIter (struct)
macro_rules! Depcrate_core_ordered_queueOrderedQueueIter {
() => {
// Module: crate::core::ordered_queue
// Provides: {"OrderedQueueIter"}
// Dependencies: {}
pub struct OrderedQueueIter < T > where T : Send , { ordering : Ordering , stop : Arc < AtomicBool > , receiver : Receiver < Ordered < T > > , receive_buffer : BinaryHeap < Ordered < T > > , pending_count : Arc < AtomicUsize > , ordered_matcher : OrderedMatcher , }
};
}
