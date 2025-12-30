// Generated macro for impl_30 (impl)
macro_rules! Depcrate_muteximpl_30 {
() => {
// Module: crate::mutex
// Provides: {"impl_30"}
// Dependencies: {}
impl < R : RawMutex , T : ? Sized + fmt :: Debug > fmt :: Debug for Mutex < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . try_lock () { Some (guard) => f . debug_struct ("Mutex") . field ("data" , & & * guard) . finish () , None => { struct LockedPlaceholder ; impl fmt :: Debug for LockedPlaceholder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("<locked>") } } f . debug_struct ("Mutex") . field ("data" , & LockedPlaceholder) . finish () } } } }
};
}
