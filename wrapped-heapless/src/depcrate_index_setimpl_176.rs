// Generated macro for impl_176 (impl)
macro_rules! Depcrate_index_setimpl_176 {
() => {
// Module: crate::index_set
// Provides: {"impl_176"}
// Dependencies: {}
impl < T , S , const N : usize > FromIterator < T > for IndexSet < T , S , N > where T : Eq + Hash , S : BuildHasher + Default , { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = T > , { let mut set = Self :: default () ; set . extend (iter) ; set } }
};
}
