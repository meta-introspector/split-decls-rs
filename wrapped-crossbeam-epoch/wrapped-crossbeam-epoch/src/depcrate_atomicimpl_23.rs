// Generated macro for impl_23 (impl)
macro_rules! Depcrate_atomicimpl_23 {
() => {
// Module: crate::atomic
// Provides: {"impl_23"}
// Dependencies: {}
impl < T , P : Pointer < T > + fmt :: Debug > fmt :: Debug for CompareExchangeError < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("CompareExchangeError") . field ("current" , & self . current) . field ("new" , & self . new) . finish () } }
};
}
