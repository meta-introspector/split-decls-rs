// Generated macro for impl_178 (impl)
macro_rules! Depcrate_sync_sharded_lockimpl_178 {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"impl_178"}
// Dependencies: {}
impl < T : ? Sized > Drop for ShardedLockWriteGuard < '_ , T > { fn drop (& mut self) { for shard in self . lock . shards . iter () . rev () { unsafe { let dest : * mut _ = shard . write_guard . get () ; let guard = (* dest) . take () ; drop (guard) ; } } } }
};
}
