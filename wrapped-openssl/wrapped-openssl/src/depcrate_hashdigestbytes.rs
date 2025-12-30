// Generated macro for DigestBytes (struct)
macro_rules! Depcrate_hashDigestBytes {
() => {
// Module: crate::hash
// Provides: {"DigestBytes"}
// Dependencies: {}
# [doc = " The resulting bytes of a digest."] # [doc = ""] # [doc = " This type derefs to a byte slice - it exists to avoid allocating memory to"] # [doc = " store the digest data."] # [derive (Copy)] pub struct DigestBytes { pub (crate) buf : [u8 ; ffi :: EVP_MAX_MD_SIZE as usize] , pub (crate) len : usize , }
};
}
