macro_rules! deps {
    () => {
        Bucket!();
    };
}

macro_rules! lock_bucket_pair {
    () => {
        deps!();
        # [doc = " Locks the two buckets for the given pair of keys and returns references to them."] # [doc = " The returned buckets must be unlocked again in order to not cause deadlocks."] # [doc = ""] # [doc = " If both keys hash to the same value, both returned references will be to the same bucket. Be"] # [doc = " careful to only unlock it once in this case, always use `unlock_bucket_pair`."] # [inline] fn lock_bucket_pair (key1 : usize , key2 : usize) -> (& 'static Bucket , & 'static Bucket) { loop { let hashtable = get_hashtable () ; let hash1 = hash (key1 , hashtable . hash_bits) ; let hash2 = hash (key2 , hashtable . hash_bits) ; let bucket1 = if hash1 <= hash2 { & hashtable . entries [hash1] } else { & hashtable . entries [hash2] } ; bucket1 . mutex . lock () ; if HASHTABLE . load (Ordering :: Relaxed) == hashtable as * const _ as * mut _ { if hash1 == hash2 { return (bucket1 , bucket1) ; } else if hash1 < hash2 { let bucket2 = & hashtable . entries [hash2] ; bucket2 . mutex . lock () ; return (bucket1 , bucket2) ; } else { let bucket2 = & hashtable . entries [hash1] ; bucket2 . mutex . lock () ; return (bucket2 , bucket1) ; } } unsafe { bucket1 . mutex . unlock () } ; } }
    };
}

lock_bucket_pair!();