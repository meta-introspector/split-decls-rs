// Generated macro for Mode (enum)
macro_rules! Depcrate_hazmatMode {
() => {
// Module: crate::hazmat
// Provides: {"Mode"}
// Dependencies: {}
# [doc = " The `mode` argument to [`merge_subtrees_root`] and friends"] # [doc = ""] # [doc = " See the [module level examples](index.html#examples)."] # [derive (Copy , Clone , Debug)] pub enum Mode < 'a > { # [doc = " Corresponding to [`hash`](crate::hash)"] Hash , # [doc = " Corresponding to [`keyed_hash`](crate::hash)"] KeyedHash (& 'a [u8 ; KEY_LEN]) , # [doc = " Corresponding to [`derive_key`](crate::hash)"] # [doc = ""] # [doc = " The [`ContextKey`] comes from [`hash_derive_key_context`]."] DeriveKeyMaterial (& 'a ContextKey) , }
};
}
