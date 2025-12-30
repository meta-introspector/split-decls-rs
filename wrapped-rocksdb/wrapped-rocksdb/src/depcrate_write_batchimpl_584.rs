// Generated macro for impl_584 (impl)
macro_rules! Depcrate_write_batchimpl_584 {
() => {
// Module: crate::write_batch
// Provides: {"impl_584"}
// Dependencies: {}
impl < const TRANSACTION : bool > Drop for WriteBatchWithTransaction < TRANSACTION > { fn drop (& mut self) { unsafe { ffi :: rocksdb_writebatch_destroy (self . inner) ; } } }
};
}
