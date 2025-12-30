// Generated macro for impl_173 (impl)
macro_rules! Depcrate_sync_sharded_lockimpl_173 {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"impl_173"}
// Dependencies: {}
impl < T : ? Sized > Deref for ShardedLockReadGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . lock . value . get () } } }
};
}
