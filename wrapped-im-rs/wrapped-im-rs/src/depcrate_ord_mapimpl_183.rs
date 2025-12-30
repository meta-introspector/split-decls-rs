// Generated macro for impl_183 (impl)
macro_rules! Depcrate_ord_mapimpl_183 {
() => {
// Module: crate::ord::map
// Provides: {"impl_183"}
// Dependencies: {}
# [cfg (has_specialisation)] impl < K , V > PartialEq for OrdMap < K , V > where K : Ord + PartialEq , V : PartialEq , { default fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . diff (other) . next () . is_none () } }
};
}
