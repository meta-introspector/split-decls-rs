// Generated macro for impl_603 (impl)
macro_rules! Depcrate_stable_hash_implsimpl_603 {
() => {
// Module: crate::stable_hash_impls
// Provides: {"impl_603"}
// Dependencies: {}
impl < HirCtx : crate :: HashStableContext > HashStable < HirCtx > for Crate < '_ > { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { let Crate { owners : _ , opt_hir_hash } = self ; opt_hir_hash . unwrap () . hash_stable (hcx , hasher) } }
};
}
