// Generated macro for impl_40 (impl)
macro_rules! Depcrate_muteximpl_40 {
() => {
// Module: crate::mutex
// Provides: {"impl_40"}
// Dependencies: {}
impl < T : fmt :: Debug + ? Sized > fmt :: Debug for Mutex < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct Locked ; impl fmt :: Debug for Locked { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("<locked>") } } match self . try_lock () { None => f . debug_struct ("Mutex") . field ("data" , & Locked) . finish () , Some (guard) => f . debug_struct ("Mutex") . field ("data" , & & * guard) . finish () , } } }
};
}
