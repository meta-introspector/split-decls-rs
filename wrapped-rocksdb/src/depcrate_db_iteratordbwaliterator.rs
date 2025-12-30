// Generated macro for DBWALIterator (struct)
macro_rules! Depcrate_db_iteratorDBWALIterator {
() => {
// Module: crate::db_iterator
// Provides: {"DBWALIterator"}
// Dependencies: {}
# [doc = " Iterates the batches of writes since a given sequence number."] # [doc = ""] # [doc = " `DBWALIterator` is returned by `DB::get_updates_since()` and will return the"] # [doc = " batches of write operations that have occurred since a given sequence number"] # [doc = " (see `DB::latest_sequence_number()`). This iterator cannot be constructed by"] # [doc = " the application."] # [doc = ""] # [doc = " The iterator item type is a tuple of (`u64`, `WriteBatch`) where the first"] # [doc = " value is the sequence number of the associated write batch."] # [doc = ""] pub struct DBWALIterator { pub (crate) inner : * mut ffi :: rocksdb_wal_iterator_t , pub (crate) start_seq_number : u64 , }
};
}
