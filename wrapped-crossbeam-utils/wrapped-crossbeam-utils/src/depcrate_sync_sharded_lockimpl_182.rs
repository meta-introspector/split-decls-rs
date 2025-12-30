// Generated macro for impl_182 (impl)
macro_rules! Depcrate_sync_sharded_lockimpl_182 {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"impl_182"}
// Dependencies: {}
impl < T : ? Sized > DerefMut for ShardedLockWriteGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . lock . value . get () } } }
};
}
