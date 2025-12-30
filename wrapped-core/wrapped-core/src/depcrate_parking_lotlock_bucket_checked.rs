// Generated macro for lock_bucket_checked (function)
macro_rules! Depcrate_parking_lotlock_bucket_checked {
() => {
// Module: crate::parking_lot
// Provides: {"lock_bucket_checked"}
// Dependencies: {}
# [doc = " Locks the bucket for the given key and returns a reference to it. But checks that the key"] # [doc = " hasn't been changed in the meantime due to a requeue."] # [doc = " The returned bucket must be unlocked again in order to not cause deadlocks."] # [inline] fn lock_bucket_checked (key : & AtomicUsize) -> (usize , & 'static Bucket) { loop { let hashtable = get_hashtable () ; let current_key = key . load (Ordering :: Relaxed) ; let hash = hash (current_key , hashtable . hash_bits) ; let bucket = & hashtable . entries [hash] ; bucket . mutex . lock () ; if HASHTABLE . load (Ordering :: Relaxed) == hashtable as * const _ as * mut _ && key . load (Ordering :: Relaxed) == current_key { return (current_key , bucket) ; } unsafe { bucket . mutex . unlock () } ; } }
};
}
