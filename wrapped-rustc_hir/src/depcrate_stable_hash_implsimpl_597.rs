// Generated macro for impl_597 (impl)
macro_rules! Depcrate_stable_hash_implsimpl_597 {
() => {
// Module: crate::stable_hash_impls
// Provides: {"impl_597"}
// Dependencies: {}
impl < HirCtx : crate :: HashStableContext > ToStableHashKey < HirCtx > for TraitItemId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & HirCtx) -> DefPathHash { self . owner_id . def_id . to_stable_hash_key (hcx) } }
};
}
