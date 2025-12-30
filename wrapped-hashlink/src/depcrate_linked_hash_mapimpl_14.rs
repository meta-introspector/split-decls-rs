// Generated macro for impl_14 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_14 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_14"}
// Dependencies: {}
impl < K : Hash + Eq , V , S : BuildHasher + Default > FromIterator < (K , V) > for LinkedHashMap < K , V , S > { # [inline] fn from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> Self { let iter = iter . into_iter () ; let mut map = Self :: with_capacity_and_hasher (iter . size_hint () . 0 , S :: default ()) ; map . extend (iter) ; map } }
};
}
