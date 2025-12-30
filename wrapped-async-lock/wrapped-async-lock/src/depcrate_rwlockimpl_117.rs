// Generated macro for impl_117 (impl)
macro_rules! Depcrate_rwlockimpl_117 {
() => {
// Module: crate::rwlock
// Provides: {"impl_117"}
// Dependencies: {}
impl < T : fmt :: Debug + ? Sized > fmt :: Debug for RwLock < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct Locked ; impl fmt :: Debug for Locked { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("<locked>") } } match self . try_read () { None => f . debug_struct ("RwLock") . field ("value" , & Locked) . finish () , Some (guard) => f . debug_struct ("RwLock") . field ("value" , & & * guard) . finish () , } } }
};
}
