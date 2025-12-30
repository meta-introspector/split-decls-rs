// Generated macro for impl_265 (impl)
macro_rules! Depcrate_ord_setimpl_265 {
() => {
// Module: crate::ord::set
// Provides: {"impl_265"}
// Dependencies: {}
impl < A : Ord + Debug > Debug for OrdSet < A > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . debug_set () . entries (self . iter ()) . finish () } }
};
}
