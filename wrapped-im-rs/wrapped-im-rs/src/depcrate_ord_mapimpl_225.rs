// Generated macro for impl_225 (impl)
macro_rules! Depcrate_ord_mapimpl_225 {
() => {
// Module: crate::ord::map
// Provides: {"impl_225"}
// Dependencies: {}
impl < 'a , K : Ord + Hash + Eq + Clone , V : Clone , S : BuildHasher > From < & 'a HashMap < K , V , S > > for OrdMap < K , V > { fn from (m : & 'a HashMap < K , V , S >) -> Self { m . iter () . map (| (k , v) | (k . clone () , v . clone ())) . collect () } }
};
}
