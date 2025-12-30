// Generated macro for ShardedLockReadGuard (struct)
macro_rules! Depcrate_sync_sharded_lockShardedLockReadGuard {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"ShardedLockReadGuard"}
// Dependencies: {}
# [doc = " A guard used to release the shared read access of a [`ShardedLock`] when dropped."] # [clippy :: has_significant_drop] pub struct ShardedLockReadGuard < 'a , T : ? Sized > { lock : & 'a ShardedLock < T > , _guard : RwLockReadGuard < 'a , () > , _marker : PhantomData < RwLockReadGuard < 'a , T > > , }
};
}
