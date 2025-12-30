// Generated macro for impl_19 (impl)
macro_rules! Depcrate_parking_lotimpl_19 {
() => {
// Module: crate::parking_lot
// Provides: {"impl_19"}
// Dependencies: {}
impl ThreadData { fn new () -> ThreadData { let num_threads = NUM_THREADS . fetch_add (1 , Ordering :: Relaxed) + 1 ; grow_hashtable (num_threads) ; ThreadData { parker : ThreadParker :: new () , key : AtomicUsize :: new (0) , next_in_queue : Cell :: new (ptr :: null ()) , unpark_token : Cell :: new (DEFAULT_UNPARK_TOKEN) , park_token : Cell :: new (DEFAULT_PARK_TOKEN) , parked_with_timeout : Cell :: new (false) , # [cfg (feature = "deadlock_detection")] deadlock_data : deadlock :: DeadlockData :: new () , } } }
};
}
