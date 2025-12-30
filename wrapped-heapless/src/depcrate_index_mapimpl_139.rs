// Generated macro for impl_139 (impl)
macro_rules! Depcrate_index_mapimpl_139 {
() => {
// Module: crate::index_map
// Provides: {"impl_139"}
// Dependencies: {}
impl < K , V , S , const N : usize > FromIterator < (K , V) > for IndexMap < K , V , S , N > where K : Eq + Hash , S : BuildHasher + Default , { fn from_iter < I > (iterable : I) -> Self where I : IntoIterator < Item = (K , V) > , { let mut map = Self :: default () ; map . extend (iterable) ; map } }
};
}
