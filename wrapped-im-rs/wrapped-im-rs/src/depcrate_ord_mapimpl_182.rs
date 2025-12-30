// Generated macro for impl_182 (impl)
macro_rules! Depcrate_ord_mapimpl_182 {
() => {
// Module: crate::ord::map
// Provides: {"impl_182"}
// Dependencies: {}
# [cfg (not (has_specialisation))] impl < K , V > PartialEq for OrdMap < K , V > where K : Ord + PartialEq , V : PartialEq , { fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . diff (other) . next () . is_none () } }
};
}
