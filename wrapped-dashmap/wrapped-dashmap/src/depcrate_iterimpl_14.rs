// Generated macro for impl_14 (impl)
macro_rules! Depcrate_iterimpl_14 {
() => {
// Module: crate::iter
// Provides: {"impl_14"}
// Dependencies: {}
impl < K : Eq + Hash , V > OwningIter < K , V > { pub (crate) fn new < S > (map : DashMap < K , V , S >) -> Self { Self { shards : map . shards . into_vec () . into_iter () , current : None , } } }
};
}
