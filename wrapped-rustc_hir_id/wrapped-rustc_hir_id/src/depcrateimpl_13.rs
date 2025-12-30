// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < CTX : HashStableContext > ToStableHashKey < CTX > for OwnerId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { hcx . def_path_hash (self . to_def_id ()) } }
};
}
