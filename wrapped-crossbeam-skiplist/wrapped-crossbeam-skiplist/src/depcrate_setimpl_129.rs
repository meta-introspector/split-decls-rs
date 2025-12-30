// Generated macro for impl_129 (impl)
macro_rules! Depcrate_setimpl_129 {
() => {
// Module: crate::set
// Provides: {"impl_129"}
// Dependencies: {}
impl < T > fmt :: Debug for Entry < '_ , T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Entry") . field ("value" , self . value ()) . finish () } }
};
}
