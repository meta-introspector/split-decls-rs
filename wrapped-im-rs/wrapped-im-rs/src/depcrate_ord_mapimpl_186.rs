// Generated macro for impl_186 (impl)
macro_rules! Depcrate_ord_mapimpl_186 {
() => {
// Module: crate::ord::map
// Provides: {"impl_186"}
// Dependencies: {}
impl < K , V > PartialOrd for OrdMap < K , V > where K : Ord , V : PartialOrd , { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
