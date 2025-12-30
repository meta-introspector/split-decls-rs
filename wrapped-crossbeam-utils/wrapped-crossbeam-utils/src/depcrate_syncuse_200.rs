// Generated macro for use_200 (pub_use)
macro_rules! Depcrate_syncuse_200 {
() => {
// Module: crate::sync
// Provides: {"use_200"}
// Dependencies: {}
# [cfg (not (crossbeam_loom))] pub use self :: sharded_lock :: { ShardedLock , ShardedLockReadGuard , ShardedLockWriteGuard } ;
};
}
