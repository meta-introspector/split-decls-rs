// Generated macro for impl_91 (impl)
macro_rules! Depcrate_infer_canonical_canonicalizerimpl_91 {
() => {
// Module: crate::infer::canonical::canonicalizer
// Provides: {"impl_91"}
// Dependencies: {}
impl CanonicalizeMode for CanonicalizeUserTypeAnnotation { fn canonicalize_free_region < 'tcx > (& self , canonicalizer : & mut Canonicalizer < '_ , 'tcx > , r : ty :: Region < 'tcx > ,) -> ty :: Region < 'tcx > { match r . kind () { ty :: ReEarlyParam (_) | ty :: ReLateParam (_) | ty :: ReErased | ty :: ReStatic | ty :: ReError (_) => r , ty :: ReVar (_) => canonicalizer . canonical_var_for_region_in_root_universe (r) , ty :: RePlaceholder (..) | ty :: ReBound (..) => { bug ! ("unexpected region in query response: `{:?}`" , r) } } } fn any (& self) -> bool { false } fn preserve_universes (& self) -> bool { false } }
};
}
