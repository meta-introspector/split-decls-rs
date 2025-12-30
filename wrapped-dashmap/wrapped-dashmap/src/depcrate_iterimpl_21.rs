// Generated macro for impl_21 (impl)
macro_rules! Depcrate_iterimpl_21 {
() => {
// Module: crate::iter
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a , K : Eq + Hash + 'a , V : 'a > Iter < 'a , K , V > { pub (crate) fn new < S > (map : & 'a DashMap < K , V , S >) -> Self { Self { shards : map . shards . iter () , current : None , } } }
};
}
