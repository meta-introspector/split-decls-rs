// Generated macro for impl_97 (impl)
macro_rules! Depcrate_builderimpl_97 {
() => {
// Module: crate::builder
// Provides: {"impl_97"}
// Dependencies: {}
impl ContentEncryptionAlgorithm { # [doc = " Return the OID of the algorithm."] pub fn oid (& self) -> ObjectIdentifier { match self { ContentEncryptionAlgorithm :: Aes128Cbc => const_oid :: db :: rfc5911 :: ID_AES_128_CBC , ContentEncryptionAlgorithm :: Aes192Cbc => const_oid :: db :: rfc5911 :: ID_AES_192_CBC , ContentEncryptionAlgorithm :: Aes256Cbc => const_oid :: db :: rfc5911 :: ID_AES_256_CBC , } } }
};
}
