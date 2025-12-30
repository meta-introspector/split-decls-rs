// Generated macro for impl_24 (impl)
macro_rules! Depcrate_iterimpl_24 {
() => {
// Module: crate::iter
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a , K : Eq + Hash + 'a , V : 'a > IterMut < 'a , K , V > { pub (crate) fn new < S > (map : & 'a DashMap < K , V , S >) -> Self { Self { shards : map . shards . iter () , current : None , } } }
};
}
