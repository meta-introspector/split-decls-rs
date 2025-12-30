// Generated macro for ToStableHashKey (trait)
macro_rules! Depcrate_stable_hasherToStableHashKey {
() => {
// Module: crate::stable_hasher
// Provides: {"ToStableHashKey"}
// Dependencies: {}
# [doc = " Implement this for types that can be turned into stable keys like, for"] # [doc = " example, for DefId that can be converted to a DefPathHash. This is used for"] # [doc = " bringing maps into a predictable order before hashing them."] pub trait ToStableHashKey < HCX > { type KeyType : Ord + Sized + HashStable < HCX > ; fn to_stable_hash_key (& self , hcx : & HCX) -> Self :: KeyType ; }
};
}
