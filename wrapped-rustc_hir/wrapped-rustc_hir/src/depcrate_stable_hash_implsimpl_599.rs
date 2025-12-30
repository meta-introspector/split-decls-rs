// Generated macro for impl_599 (impl)
macro_rules! Depcrate_stable_hash_implsimpl_599 {
() => {
// Module: crate::stable_hash_impls
// Provides: {"impl_599"}
// Dependencies: {}
impl < HirCtx : crate :: HashStableContext > ToStableHashKey < HirCtx > for ForeignItemId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & HirCtx) -> DefPathHash { self . owner_id . def_id . to_stable_hash_key (hcx) } }
};
}
