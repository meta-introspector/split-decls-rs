// Generated macro for impl_256 (impl)
macro_rules! Depcrate_ord_setimpl_256 {
() => {
// Module: crate::ord::set
// Provides: {"impl_256"}
// Dependencies: {}
impl < A : Ord > Ord for OrdSet < A > { fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
