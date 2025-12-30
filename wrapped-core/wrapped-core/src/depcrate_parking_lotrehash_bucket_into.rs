// Generated macro for rehash_bucket_into (function)
macro_rules! Depcrate_parking_lotrehash_bucket_into {
() => {
// Module: crate::parking_lot
// Provides: {"rehash_bucket_into"}
// Dependencies: {}
# [doc = " Iterate through all `ThreadData` objects in the bucket and insert them into the given table"] # [doc = " in the bucket their key correspond to for this table."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The given `bucket` must have a correctly constructed linked list under `queue_head`, containing"] # [doc = " `ThreadData` instances that must stay valid at least as long as the given `table` is in use."] # [doc = ""] # [doc = " The given `table` must only contain buckets with correctly constructed linked lists."] unsafe fn rehash_bucket_into (bucket : & 'static Bucket , table : & mut HashTable) { let mut current : * const ThreadData = bucket . queue_head . get () ; while ! current . is_null () { let next = (* current) . next_in_queue . get () ; let hash = hash ((* current) . key . load (Ordering :: Relaxed) , table . hash_bits) ; if table . entries [hash] . queue_tail . get () . is_null () { table . entries [hash] . queue_head . set (current) ; } else { (* table . entries [hash] . queue_tail . get ()) . next_in_queue . set (current) ; } table . entries [hash] . queue_tail . set (current) ; (* current) . next_in_queue . set (ptr :: null ()) ; current = next ; } }
};
}
