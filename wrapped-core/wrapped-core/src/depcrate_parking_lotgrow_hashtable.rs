// Generated macro for grow_hashtable (function)
macro_rules! Depcrate_parking_lotgrow_hashtable {
() => {
// Module: crate::parking_lot
// Provides: {"grow_hashtable"}
// Dependencies: {}
fn grow_hashtable (num_threads : usize) { let old_table = loop { let table = get_hashtable () ; if table . entries . len () >= LOAD_FACTOR * num_threads { return ; } for bucket in & table . entries [..] { bucket . mutex . lock () ; } if HASHTABLE . load (Ordering :: Relaxed) == table as * const _ as * mut _ { break table ; } for bucket in & table . entries [..] { unsafe { bucket . mutex . unlock () } ; } } ; let mut new_table = HashTable :: new (num_threads , old_table) ; for bucket in & old_table . entries [..] { unsafe { rehash_bucket_into (bucket , & mut new_table) } ; } HASHTABLE . store (Box :: into_raw (new_table) , Ordering :: Release) ; for bucket in & old_table . entries [..] { unsafe { bucket . mutex . unlock () } ; } }
};
}
