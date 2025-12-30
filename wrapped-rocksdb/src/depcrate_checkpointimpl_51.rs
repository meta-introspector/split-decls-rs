// Generated macro for impl_51 (impl)
macro_rules! Depcrate_checkpointimpl_51 {
() => {
// Module: crate::checkpoint
// Provides: {"impl_51"}
// Dependencies: {}
impl Drop for Checkpoint < '_ > { fn drop (& mut self) { unsafe { ffi :: rocksdb_checkpoint_object_destroy (self . inner) ; } } }
};
}
