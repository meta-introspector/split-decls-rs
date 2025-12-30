// Generated macro for impl_805 (impl)
macro_rules! Depcrate_prefixesimpl_805 {
() => {
// Module: crate::prefixes
// Provides: {"impl_805"}
// Dependencies: {}
impl < 'tcx > IsPrefixOf < 'tcx > for PlaceRef < 'tcx > { fn is_prefix_of (& self , other : PlaceRef < 'tcx >) -> bool { self . local == other . local && self . projection . len () <= other . projection . len () && self . projection == & other . projection [.. self . projection . len ()] } }
};
}
