// Generated macro for macro_210 (macro)
macro_rules! Depcrate_ciphermacro_210 {
() => {
// Module: crate::cipher
// Provides: {"macro_210"}
// Dependencies: {}
cfg_if ! { if # [cfg (ossl300)] { use foreign_types :: ForeignType ; type Inner = * mut ffi :: EVP_CIPHER ; impl Drop for Cipher { # [inline] fn drop (& mut self) { unsafe { ffi :: EVP_CIPHER_free (self . as_ptr ()) ; } } } impl ForeignType for Cipher { type CType = ffi :: EVP_CIPHER ; type Ref = CipherRef ; # [inline] unsafe fn from_ptr (ptr : * mut Self :: CType) -> Self { Cipher (ptr) } # [inline] fn as_ptr (& self) -> * mut Self :: CType { self . 0 } } impl Deref for Cipher { type Target = CipherRef ; # [inline] fn deref (& self) -> & Self :: Target { unsafe { CipherRef :: from_ptr (self . as_ptr ()) } } } impl DerefMut for Cipher { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { CipherRef :: from_ptr_mut (self . as_ptr ()) } } } } else { enum Inner { } impl Deref for Cipher { type Target = CipherRef ; # [inline] fn deref (& self) -> & Self :: Target { match self . 0 { } } } impl DerefMut for Cipher { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { match self . 0 { } } } } }
};
}
