// Generated macro for table_entry (function)
macro_rules! Depcrate_shardedtable_entry {
() => {
// Module: crate::sharded
// Provides: {"table_entry"}
// Dependencies: {}
# [inline] fn table_entry < 'a , K , V , Q > (table : & 'a mut HashTable < (K , V) > , hash : u64 , key : & Q ,) -> Entry < 'a , (K , V) > where K : Hash + Borrow < Q > , Q : ? Sized + Eq , { table . entry (hash , move | (k , _) | k . borrow () == key , | (k , _) | make_hash (k)) }
};
}
