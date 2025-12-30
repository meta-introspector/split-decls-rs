// Generated macro for impl_131 (impl)
macro_rules! Depcrate_index_mapimpl_131 {
() => {
// Module: crate::index_map
// Provides: {"impl_131"}
// Dependencies: {}
impl < K , Q , V , S , const N : usize > ops :: IndexMut < & Q > for IndexMap < K , V , S , N > where K : Eq + Hash + Borrow < Q > , Q : ? Sized + Eq + Hash , S : BuildHasher , { fn index_mut (& mut self , key : & Q) -> & mut V { self . get_mut (key) . expect ("key not found") } }
};
}
