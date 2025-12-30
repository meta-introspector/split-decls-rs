// Generated macro for impl_130 (impl)
macro_rules! Depcrate_index_mapimpl_130 {
() => {
// Module: crate::index_map
// Provides: {"impl_130"}
// Dependencies: {}
impl < K , Q , V , S , const N : usize > ops :: Index < & Q > for IndexMap < K , V , S , N > where K : Eq + Hash + Borrow < Q > , Q : ? Sized + Eq + Hash , S : BuildHasher , { type Output = V ; fn index (& self , key : & Q) -> & V { self . get (key) . expect ("key not found") } }
};
}
