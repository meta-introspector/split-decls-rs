// Generated macro for impl_137 (impl)
macro_rules! Depcrate_index_mapimpl_137 {
() => {
// Module: crate::index_map
// Provides: {"impl_137"}
// Dependencies: {}
impl < K , V , S , const N : usize > Extend < (K , V) > for IndexMap < K , V , S , N > where K : Eq + Hash , S : BuildHasher , { fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = (K , V) > , { for (k , v) in iterable { self . insert (k , v) . ok () . unwrap () ; } } }
};
}
