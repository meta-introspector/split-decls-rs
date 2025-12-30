// Generated macro for impl_95 (impl)
macro_rules! Depcrate_infer_canonical_canonicalizerimpl_95 {
() => {
// Module: crate::infer::canonical::canonicalizer
// Provides: {"impl_95"}
// Dependencies: {}
impl CanonicalizeMode for CanonicalizeFreeRegionsOtherThanStatic { fn canonicalize_free_region < 'tcx > (& self , canonicalizer : & mut Canonicalizer < '_ , 'tcx > , r : ty :: Region < 'tcx > ,) -> ty :: Region < 'tcx > { if r . is_static () { r } else { canonicalizer . canonical_var_for_region_in_root_universe (r) } } fn any (& self) -> bool { true } fn preserve_universes (& self) -> bool { false } }
};
}
