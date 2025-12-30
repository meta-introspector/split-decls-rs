// Generated macro for impl_191 (impl)
macro_rules! Depcrate_dyn_mapimpl_191 {
() => {
// Module: crate::dyn_map
// Provides: {"impl_191"}
// Dependencies: {}
impl < K : Hash + Eq + 'static , V : 'static > Policy for (K , V) { type K = K ; type V = V ; fn insert (map : & mut DynMap , key : K , value : V) { map . map . entry :: < FxHashMap < K , V > > () . or_insert_with (Default :: default) . insert (key , value) ; } fn get < 'a > (map : & 'a DynMap , key : & K) -> Option < & 'a V > { map . map . get :: < FxHashMap < K , V > > () ? . get (key) } fn is_empty (map : & DynMap) -> bool { map . map . get :: < FxHashMap < K , V > > () . is_none_or (| it | it . is_empty ()) } }
};
}
