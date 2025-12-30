// Generated macro for impl_255 (impl)
macro_rules! Depcrate_ord_setimpl_255 {
() => {
// Module: crate::ord::set
// Provides: {"impl_255"}
// Dependencies: {}
impl < A : Ord > PartialOrd for OrdSet < A > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
