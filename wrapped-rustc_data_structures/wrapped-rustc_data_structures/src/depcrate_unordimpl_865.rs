// Generated macro for impl_865 (impl)
macro_rules! Depcrate_unordimpl_865 {
() => {
// Module: crate::unord
// Provides: {"impl_865"}
// Dependencies: {}
impl < V : Hash + Eq > FromIterator < V > for UnordSet < V > { # [inline] fn from_iter < T : IntoIterator < Item = V > > (iter : T) -> Self { UnordSet { inner : FxHashSet :: from_iter (iter) } } }
};
}
