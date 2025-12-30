// Generated macro for impl_482 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_482 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_482"}
// Dependencies: {}
# [stable (feature = "extend_ref" , since = "1.2.0")] impl < 'a , K : Ord + Copy , V : Copy , A : Allocator + Clone > Extend < (& 'a K , & 'a V) > for BTreeMap < K , V , A > { fn extend < I : IntoIterator < Item = (& 'a K , & 'a V) > > (& mut self , iter : I) { self . extend (iter . into_iter () . map (| (& key , & value) | (key , value))) ; } # [inline] fn extend_one (& mut self , (& k , & v) : (& 'a K , & 'a V)) { self . insert (k , v) ; } }
};
}
