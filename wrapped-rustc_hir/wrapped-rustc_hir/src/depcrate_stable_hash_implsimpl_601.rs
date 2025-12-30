// Generated macro for impl_601 (impl)
macro_rules! Depcrate_stable_hash_implsimpl_601 {
() => {
// Module: crate::stable_hash_impls
// Provides: {"impl_601"}
// Dependencies: {}
impl < HirCtx : crate :: HashStableContext > HashStable < HirCtx > for DelayedLints { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { let DelayedLints { opt_hash , .. } = * self ; opt_hash . unwrap () . hash_stable (hcx , hasher) ; } }
};
}
