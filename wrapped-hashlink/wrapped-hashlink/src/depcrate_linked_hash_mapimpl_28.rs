// Generated macro for impl_28 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_28 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a , K , V , S > Extend < (& 'a K , & 'a V) > for LinkedHashMap < K , V , S > where K : 'a + Hash + Eq + Copy , V : 'a + Copy , S : BuildHasher , { # [inline] fn extend < I : IntoIterator < Item = (& 'a K , & 'a V) > > (& mut self , iter : I) { for (& k , & v) in iter { self . insert (k , v) ; } } }
};
}
