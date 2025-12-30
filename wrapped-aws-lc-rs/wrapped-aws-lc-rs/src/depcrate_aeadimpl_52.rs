// Generated macro for impl_52 (impl)
macro_rules! Depcrate_aeadimpl_52 {
() => {
// Module: crate::aead
// Provides: {"impl_52"}
// Dependencies: {}
impl Algorithm { # [doc = " The length of the key."] # [inline] # [must_use] pub fn key_len (& self) -> usize { self . key_len } # [doc = " The length of a tag."] # [doc = ""] # [doc = " See also `MAX_TAG_LEN`."] # [inline] # [must_use] pub fn tag_len (& self) -> usize { TAG_LEN } # [doc = " The length of the nonces."] # [inline] # [must_use] pub fn nonce_len (& self) -> usize { NONCE_LEN } }
};
}
