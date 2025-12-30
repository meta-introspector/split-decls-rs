// Generated macro for worker_thread_fn (function)
macro_rules! Depcrateworker_thread_fn {
() => {
// Module: crate
// Provides: {"worker_thread_fn"}
// Dependencies: {}
# [doc = " Function executed in worker-threads. Infinite loop generating and testing patterns."] fn worker_thread_fn (literals : & Vec < Vec < u8 > > , num_batches_finished : & AtomicU64 , mut rng : Xoshiro256Plus , pattern_time : & Mutex < (Pattern , Instant) > ,) { let pattern_distr = UniformPatterns :: new (& literals) ; loop { for _ in 0 .. BATCH_SIZE { let pattern = pattern_distr . sample (& mut rng) ; { * pattern_time . lock () . unwrap () = (pattern . clone () , Instant :: now ()) ; } let _ = test_catch_unwind (& pattern) ; } let _ = num_batches_finished . fetch_add (1 , Ordering :: Relaxed) ; } }
};
}
