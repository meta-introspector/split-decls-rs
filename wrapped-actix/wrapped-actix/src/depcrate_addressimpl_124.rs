// Generated macro for impl_124 (impl)
macro_rules! Depcrate_addressimpl_124 {
() => {
// Module: crate::address
// Provides: {"impl_124"}
// Dependencies: {}
impl < A : Actor > fmt :: Debug for WeakAddr < A > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("WeakAddr") . field ("wtx" , & self . wtx) . finish () } }
};
}
