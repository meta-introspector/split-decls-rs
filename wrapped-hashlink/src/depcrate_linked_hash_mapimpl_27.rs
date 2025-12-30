// Generated macro for impl_27 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_27 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_27"}
// Dependencies: {}
impl < K : Hash + Eq , V , S : BuildHasher > Extend < (K , V) > for LinkedHashMap < K , V , S > { # [inline] fn extend < I : IntoIterator < Item = (K , V) > > (& mut self , iter : I) { for (k , v) in iter { self . insert (k , v) ; } } }
};
}
