// Generated macro for impl_56 (impl)
macro_rules! Depcrate_ieeeimpl_56 {
() => {
// Module: crate::ieee
// Provides: {"impl_56"}
// Dependencies: {}
impl < S : Semantics > fmt :: Debug for IeeeFloat < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}({:?} | {}{:?} * 2^{})" , self , self . category () , if self . is_negative () { "-" } else { "+" } , self . sig , self . exp) } }
};
}
