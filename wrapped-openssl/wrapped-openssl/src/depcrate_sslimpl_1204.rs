// Generated macro for impl_1204 (impl)
macro_rules! Depcrate_sslimpl_1204 {
() => {
// Module: crate::ssl
// Provides: {"impl_1204"}
// Dependencies: {}
impl ForeignType for SslCipher { type CType = ffi :: SSL_CIPHER ; type Ref = SslCipherRef ; # [inline] unsafe fn from_ptr (ptr : * mut ffi :: SSL_CIPHER) -> SslCipher { SslCipher (ptr) } # [inline] fn as_ptr (& self) -> * mut ffi :: SSL_CIPHER { self . 0 } }
};
}
