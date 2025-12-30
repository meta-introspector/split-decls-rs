// Generated macro for impl_138 (impl)
macro_rules! Depcrate_index_mapimpl_138 {
() => {
// Module: crate::index_map
// Provides: {"impl_138"}
// Dependencies: {}
impl < 'a , K , V , S , const N : usize > Extend < (& 'a K , & 'a V) > for IndexMap < K , V , S , N > where K : Eq + Hash + Copy , V : Copy , S : BuildHasher , { fn extend < I > (& mut self , iterable : I) where I : IntoIterator < Item = (& 'a K , & 'a V) > , { self . extend (iterable . into_iter () . map (| (& key , & value) | (key , value))) ; } }
};
}
