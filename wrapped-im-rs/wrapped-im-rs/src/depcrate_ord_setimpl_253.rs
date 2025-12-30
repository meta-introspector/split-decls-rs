// Generated macro for impl_253 (impl)
macro_rules! Depcrate_ord_setimpl_253 {
() => {
// Module: crate::ord::set
// Provides: {"impl_253"}
// Dependencies: {}
impl < A : Ord > PartialEq for OrdSet < A > { fn eq (& self , other : & Self) -> bool { PoolRef :: ptr_eq (& self . root , & other . root) || (self . len () == other . len () && self . diff (other) . next () . is_none ()) } }
};
}
