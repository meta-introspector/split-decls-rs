// Generated macro for impl_187 (impl)
macro_rules! Depcrate_ord_mapimpl_187 {
() => {
// Module: crate::ord::map
// Provides: {"impl_187"}
// Dependencies: {}
impl < K , V > Ord for OrdMap < K , V > where K : Ord , V : Ord , { fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
