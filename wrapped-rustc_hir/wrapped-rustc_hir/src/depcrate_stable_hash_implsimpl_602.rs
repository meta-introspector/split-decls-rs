// Generated macro for impl_602 (impl)
macro_rules! Depcrate_stable_hash_implsimpl_602 {
() => {
// Module: crate::stable_hash_impls
// Provides: {"impl_602"}
// Dependencies: {}
impl < 'tcx , HirCtx : crate :: HashStableContext > HashStable < HirCtx > for AttributeMap < 'tcx > { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { let AttributeMap { opt_hash , define_opaque : _ , map : _ } = * self ; opt_hash . unwrap () . hash_stable (hcx , hasher) ; } }
};
}
