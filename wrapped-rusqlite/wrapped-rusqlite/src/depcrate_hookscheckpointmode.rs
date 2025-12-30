// Generated macro for CheckpointMode (enum)
macro_rules! Depcrate_hooksCheckpointMode {
() => {
// Module: crate::hooks
// Provides: {"CheckpointMode"}
// Dependencies: {}
# [doc = " Checkpoint mode"] # [derive (Clone , Copy)] # [repr (i32)] # [non_exhaustive] pub enum CheckpointMode { # [doc = " Do as much as possible w/o blocking"] PASSIVE = ffi :: SQLITE_CHECKPOINT_PASSIVE , # [doc = " Wait for writers, then checkpoint"] FULL = ffi :: SQLITE_CHECKPOINT_FULL , # [doc = " Like FULL but wait for readers"] RESTART = ffi :: SQLITE_CHECKPOINT_RESTART , # [doc = " Like RESTART but also truncate WAL"] TRUNCATE = ffi :: SQLITE_CHECKPOINT_TRUNCATE , # [doc = " Do no work at all"] # [cfg (feature = "modern_sqlite")] NOOP = - 1 , }
};
}
