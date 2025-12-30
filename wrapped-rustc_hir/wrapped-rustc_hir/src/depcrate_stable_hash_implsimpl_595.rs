// Generated macro for impl_595 (impl)
macro_rules! Depcrate_stable_hash_implsimpl_595 {
() => {
// Module: crate::stable_hash_impls
// Provides: {"impl_595"}
// Dependencies: {}
impl < HirCtx : crate :: HashStableContext > ToStableHashKey < HirCtx > for BodyId { type KeyType = (DefPathHash , ItemLocalId) ; # [inline] fn to_stable_hash_key (& self , hcx : & HirCtx) -> (DefPathHash , ItemLocalId) { let BodyId { hir_id } = * self ; hir_id . to_stable_hash_key (hcx) } }
};
}
