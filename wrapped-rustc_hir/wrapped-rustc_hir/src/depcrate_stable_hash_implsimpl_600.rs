// Generated macro for impl_600 (impl)
macro_rules! Depcrate_stable_hash_implsimpl_600 {
() => {
// Module: crate::stable_hash_impls
// Provides: {"impl_600"}
// Dependencies: {}
impl < 'tcx , HirCtx : crate :: HashStableContext > HashStable < HirCtx > for OwnerNodes < 'tcx > { fn hash_stable (& self , hcx : & mut HirCtx , hasher : & mut StableHasher) { let OwnerNodes { opt_hash_including_bodies , nodes : _ , bodies : _ } = * self ; opt_hash_including_bodies . unwrap () . hash_stable (hcx , hasher) ; } }
};
}
