// Generated macro for impl_362 (impl)
macro_rules! Depcrate_hash_mapimpl_362 {
() => {
// Module: crate::hash::map
// Provides: {"impl_362"}
// Dependencies: {}
impl < K , V , S > FromIterator < (K , V) > for HashMap < K , V , S > where K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Default , { fn from_iter < T > (i : T) -> Self where T : IntoIterator < Item = (K , V) > , { let mut map = Self :: default () ; for (k , v) in i { map . insert (k , v) ; } map } }
};
}
