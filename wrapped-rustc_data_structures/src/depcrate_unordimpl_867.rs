// Generated macro for impl_867 (impl)
macro_rules! Depcrate_unordimpl_867 {
() => {
// Module: crate::unord
// Provides: {"impl_867"}
// Dependencies: {}
impl < V : Hash + Eq , I : Iterator < Item = V > > From < UnordItems < V , I > > for UnordSet < V > { fn from (value : UnordItems < V , I >) -> Self { UnordSet { inner : FxHashSet :: from_iter (value . 0) } } }
};
}
