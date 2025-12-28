macro_rules! deps {
    () => {
        ThreadData!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl ThreadData { fn new () -> ThreadData { let num_threads = NUM_THREADS . fetch_add (1 , Ordering :: Relaxed) + 1 ; grow_hashtable (num_threads) ; ThreadData { parker : ThreadParker :: new () , key : AtomicUsize :: new (0) , next_in_queue : Cell :: new (ptr :: null ()) , unpark_token : Cell :: new (DEFAULT_UNPARK_TOKEN) , park_token : Cell :: new (DEFAULT_PARK_TOKEN) , parked_with_timeout : Cell :: new (false) , # [cfg (feature = "deadlock_detection")] deadlock_data : deadlock :: DeadlockData :: new () , } } }
    };
}

impl_11!();