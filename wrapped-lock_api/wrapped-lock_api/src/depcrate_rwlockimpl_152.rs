// Generated macro for impl_152 (impl)
macro_rules! Depcrate_rwlockimpl_152 {
() => {
// Module: crate::rwlock
// Provides: {"impl_152"}
// Dependencies: {}
impl < R : RawRwLock , T : ? Sized + fmt :: Debug > fmt :: Debug for RwLock < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_struct ("RwLock") ; match self . try_read () { Some (guard) => d . field ("data" , & & * guard) , None => { d . field ("data" , & format_args ! ("<locked>")) } } ; d . finish () } }
};
}
