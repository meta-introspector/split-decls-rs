// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl < CTX : rustc_span :: HashStableContext > ToStableHashKey < CTX > for HirId { type KeyType = (DefPathHash , ItemLocalId) ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> (DefPathHash , ItemLocalId) { let def_path_hash = self . owner . def_id . to_stable_hash_key (hcx) ; (def_path_hash , self . local_id) } }
};
}
