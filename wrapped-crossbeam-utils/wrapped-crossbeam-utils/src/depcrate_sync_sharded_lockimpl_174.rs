// Generated macro for impl_174 (impl)
macro_rules! Depcrate_sync_sharded_lockimpl_174 {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"impl_174"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for ShardedLockReadGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ShardedLockReadGuard") . field ("lock" , & self . lock) . finish () } }
};
}
