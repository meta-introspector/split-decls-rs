macro_rules! deps {
    () => {
        Bucket!();
    };
}

macro_rules! lock_bucket {
    () => {
        deps!();
        # [doc = " Locks the bucket for the given key and returns a reference to it."] # [doc = " The returned bucket must be unlocked again in order to not cause deadlocks."] # [inline] fn lock_bucket (key : usize) -> & 'static Bucket { loop { let hashtable = get_hashtable () ; let hash = hash (key , hashtable . hash_bits) ; let bucket = & hashtable . entries [hash] ; bucket . mutex . lock () ; if HASHTABLE . load (Ordering :: Relaxed) == hashtable as * const _ as * mut _ { return bucket ; } unsafe { bucket . mutex . unlock () } ; } }
    };
}

lock_bucket!()