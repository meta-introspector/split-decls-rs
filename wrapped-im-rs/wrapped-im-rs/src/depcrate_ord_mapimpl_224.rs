// Generated macro for impl_224 (impl)
macro_rules! Depcrate_ord_mapimpl_224 {
() => {
// Module: crate::ord::map
// Provides: {"impl_224"}
// Dependencies: {}
impl < K : Ord + Hash + Eq + Clone , V : Clone , S : BuildHasher > From < HashMap < K , V , S > > for OrdMap < K , V > { fn from (m : HashMap < K , V , S >) -> Self { m . into_iter () . collect () } }
};
}
