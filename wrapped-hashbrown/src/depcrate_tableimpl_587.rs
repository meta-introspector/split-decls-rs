// Generated macro for impl_587 (impl)
macro_rules! Depcrate_tableimpl_587 {
() => {
// Module: crate::table
// Provides: {"impl_587"}
// Dependencies: {}
impl < 'a , T > Clone for Iter < 'a , T > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Iter < 'a , T > { Iter { inner : self . inner . clone () , marker : PhantomData , } } }
};
}
