// Generated macro for impl_93 (impl)
macro_rules! Depcrate_infer_canonical_canonicalizerimpl_93 {
() => {
// Module: crate::infer::canonical::canonicalizer
// Provides: {"impl_93"}
// Dependencies: {}
impl CanonicalizeMode for CanonicalizeAllFreeRegions { fn canonicalize_free_region < 'tcx > (& self , canonicalizer : & mut Canonicalizer < '_ , 'tcx > , r : ty :: Region < 'tcx > ,) -> ty :: Region < 'tcx > { canonicalizer . canonical_var_for_region_in_root_universe (r) } fn any (& self) -> bool { true } fn preserve_universes (& self) -> bool { false } }
};
}
