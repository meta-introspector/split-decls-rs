// Generated macro for ShardedLockWriteGuard (struct)
macro_rules! Depcrate_sync_sharded_lockShardedLockWriteGuard {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"ShardedLockWriteGuard"}
// Dependencies: {}
# [doc = " A guard used to release the exclusive write access of a [`ShardedLock`] when dropped."] # [clippy :: has_significant_drop] pub struct ShardedLockWriteGuard < 'a , T : ? Sized > { lock : & 'a ShardedLock < T > , _marker : PhantomData < RwLockWriteGuard < 'a , T > > , }
};
}
