// Generated macro for impl_168 (impl)
macro_rules! Depcrate_sync_sharded_lockimpl_168 {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"impl_168"}
// Dependencies: {}
impl < T : ? Sized + fmt :: Debug > fmt :: Debug for ShardedLock < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_read () { Ok (guard) => f . debug_struct ("ShardedLock") . field ("data" , & & * guard) . finish () , Err (TryLockError :: Poisoned (err)) => f . debug_struct ("ShardedLock") . field ("data" , & & * * err . get_ref ()) . finish () , Err (TryLockError :: WouldBlock) => { struct LockedPlaceholder ; impl fmt :: Debug for LockedPlaceholder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("<locked>") } } f . debug_struct ("ShardedLock") . field ("data" , & LockedPlaceholder) . finish () } } } }
};
}
