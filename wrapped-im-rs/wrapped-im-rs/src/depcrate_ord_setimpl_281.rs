// Generated macro for impl_281 (impl)
macro_rules! Depcrate_ord_setimpl_281 {
() => {
// Module: crate::ord::set
// Provides: {"impl_281"}
// Dependencies: {}
impl < 'a , A > From < & 'a [A] > for OrdSet < A > where A : Ord + Clone , { fn from (slice : & 'a [A]) -> Self { slice . iter () . cloned () . collect () } }
};
}
