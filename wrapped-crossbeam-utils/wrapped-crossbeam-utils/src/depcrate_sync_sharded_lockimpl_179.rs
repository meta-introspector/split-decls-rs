// Generated macro for impl_179 (impl)
macro_rules! Depcrate_sync_sharded_lockimpl_179 {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"impl_179"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for ShardedLockWriteGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ShardedLockWriteGuard") . field ("lock" , & self . lock) . finish () } }
};
}
