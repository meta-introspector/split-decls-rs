// Generated macro for new_ordered_queue (function)
macro_rules! Depcrate_core_ordered_queuenew_ordered_queue {
() => {
// Module: crate::core::ordered_queue
// Provides: {"new_ordered_queue"}
// Dependencies: {}
pub (crate) fn new_ordered_queue < T > (stop : Arc < AtomicBool > , ordering : Ordering ,) -> (OrderedQueue < T > , OrderedQueueIter < T >) where T : Send , { let pending_count = Arc :: new (AtomicUsize :: new (0)) ; let (sender , receiver) = channel :: unbounded () ; (OrderedQueue { sender , pending_count : pending_count . clone () , stop : stop . clone () , } , OrderedQueueIter { ordering , receiver , ordered_matcher : OrderedMatcher :: default () , receive_buffer : BinaryHeap :: new () , pending_count , stop , } ,) }
};
}
