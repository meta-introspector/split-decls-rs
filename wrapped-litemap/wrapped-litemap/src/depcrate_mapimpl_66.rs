// Generated macro for impl_66 (impl)
macro_rules! Depcrate_mapimpl_66 {
() => {
// Module: crate::map
// Provides: {"impl_66"}
// Dependencies: {}
impl < K , V , S > Extend < (K , V) > for LiteMap < K , V , S > where K : Ord , S : StoreBulkMut < K , V > , { fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { self . values . lm_extend (iter) } }
};
}
