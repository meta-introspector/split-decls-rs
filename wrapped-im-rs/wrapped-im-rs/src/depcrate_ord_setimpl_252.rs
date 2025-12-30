// Generated macro for impl_252 (impl)
macro_rules! Depcrate_ord_setimpl_252 {
() => {
// Module: crate::ord::set
// Provides: {"impl_252"}
// Dependencies: {}
impl < A > Clone for OrdSet < A > { # [doc = " Clone a set."] # [doc = ""] # [doc = " Time: O(1)"] # [inline] fn clone (& self) -> Self { OrdSet { size : self . size , pool : self . pool . clone () , root : self . root . clone () , } } }
};
}
