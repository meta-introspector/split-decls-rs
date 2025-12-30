// Generated macro for impl_187 (impl)
macro_rules! Depcrate_db_iteratorimpl_187 {
() => {
// Module: crate::db_iterator
// Provides: {"impl_187"}
// Dependencies: {}
impl Iterator for DBWALIterator { type Item = Result < (u64 , WriteBatch) , Error > ; fn next (& mut self) -> Option < Self :: Item > { if ! self . valid () { return None ; } let mut seq : u64 = 0 ; let mut batch = WriteBatch { inner : unsafe { ffi :: rocksdb_wal_iter_get_batch (self . inner , & mut seq) } , } ; while seq <= self . start_seq_number { unsafe { ffi :: rocksdb_wal_iter_next (self . inner) ; } if ! self . valid () { return None ; } batch = WriteBatch { inner : unsafe { ffi :: rocksdb_wal_iter_get_batch (self . inner , & mut seq) } , } ; } if ! self . valid () { return self . status () . err () . map (Result :: Err) ; } unsafe { ffi :: rocksdb_wal_iter_next (self . inner) ; } Some (Ok ((seq , batch))) } }
};
}
