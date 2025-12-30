// Generated macro for count_handler (function)
macro_rules! Depcrate_thread_pool_testcount_handler {
() => {
// Module: crate::thread_pool::test
// Provides: {"count_handler"}
// Dependencies: {}
# [doc = " Creates a start/exit handler that increments an atomic counter."] fn count_handler () -> (Arc < AtomicUsize > , impl Fn (usize)) { let count = Arc :: new (AtomicUsize :: new (0)) ; (Arc :: clone (& count) , move | _ | { count . fetch_add (1 , Ordering :: SeqCst) ; }) }
};
}
