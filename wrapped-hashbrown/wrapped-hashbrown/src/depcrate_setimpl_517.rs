// Generated macro for impl_517 (impl)
macro_rules! Depcrate_setimpl_517 {
() => {
// Module: crate::set
// Provides: {"impl_517"}
// Dependencies: {}
impl < K : fmt :: Debug > fmt :: Debug for Iter < '_ , K > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
