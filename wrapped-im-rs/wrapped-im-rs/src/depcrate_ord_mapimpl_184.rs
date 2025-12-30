// Generated macro for impl_184 (impl)
macro_rules! Depcrate_ord_mapimpl_184 {
() => {
// Module: crate::ord::map
// Provides: {"impl_184"}
// Dependencies: {}
# [cfg (has_specialisation)] impl < K , V > PartialEq for OrdMap < K , V > where K : Ord + Eq , V : Eq , { fn eq (& self , other : & Self) -> bool { PoolRef :: ptr_eq (& self . root , & other . root) || (self . len () == other . len () && self . diff (other) . next () . is_none ()) } }
};
}
