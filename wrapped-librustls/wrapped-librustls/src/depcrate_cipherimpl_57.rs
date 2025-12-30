// Generated macro for impl_57 (impl)
macro_rules! Depcrate_cipherimpl_57 {
() => {
// Module: crate::cipher
// Provides: {"impl_57"}
// Dependencies: {}
impl rustls_supported_ciphersuite { # [doc = " Return a 16-bit unsigned integer corresponding to this cipher suite's assignment from"] # [doc = " <https://www.iana.org/assignments/tls-parameters/tls-parameters.xhtml#tls-parameters-4>."] # [doc = ""] # [doc = " The bytes from the assignment are interpreted in network order."] # [no_mangle] pub extern "C" fn rustls_supported_ciphersuite_get_suite (supported_ciphersuite : * const rustls_supported_ciphersuite ,) -> u16 { let supported_ciphersuite = try_ref_from_ptr ! (supported_ciphersuite) ; u16 :: from (match supported_ciphersuite { SupportedCipherSuite :: Tls12 (sc) => & sc . common , SupportedCipherSuite :: Tls13 (sc) => & sc . common , } . suite ,) } }
};
}
