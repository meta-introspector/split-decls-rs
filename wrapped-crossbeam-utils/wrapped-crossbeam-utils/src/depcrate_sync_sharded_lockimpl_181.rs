// Generated macro for impl_181 (impl)
macro_rules! Depcrate_sync_sharded_lockimpl_181 {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"impl_181"}
// Dependencies: {}
impl < T : ? Sized > Deref for ShardedLockWriteGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . lock . value . get () } } }
};
}
