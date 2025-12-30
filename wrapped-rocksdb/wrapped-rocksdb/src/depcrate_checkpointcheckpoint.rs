// Generated macro for Checkpoint (struct)
macro_rules! Depcrate_checkpointCheckpoint {
() => {
// Module: crate::checkpoint
// Provides: {"Checkpoint"}
// Dependencies: {}
# [doc = " Database's checkpoint object."] # [doc = " Used to create checkpoints of the specified DB from time to time."] pub struct Checkpoint < 'db > { inner : * mut ffi :: rocksdb_checkpoint_t , _db : PhantomData < & 'db () > , }
};
}
