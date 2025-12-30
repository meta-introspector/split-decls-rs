// Generated macro for extend (function)
macro_rules! Depcrate_external_trait_impls_rayon_mapextend {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"extend"}
// Dependencies: {}
fn extend < K , V , S , A , I > (map : & mut HashMap < K , V , S , A > , par_iter : I) where K : Eq + Hash , S : BuildHasher , I : IntoParallelIterator , A : Allocator , HashMap < K , V , S , A > : Extend < I :: Item > , { let (list , len) = super :: helpers :: collect (par_iter) ; let reserve = if map . is_empty () { len } else { (len + 1) / 2 } ; map . reserve (reserve) ; for vec in list { map . extend (vec) ; } }
};
}
