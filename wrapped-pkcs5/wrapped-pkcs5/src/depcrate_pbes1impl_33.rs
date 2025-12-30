// Generated macro for impl_33 (impl)
macro_rules! Depcrate_pbes1impl_33 {
() => {
// Module: crate::pbes1
// Provides: {"impl_33"}
// Dependencies: {}
impl EncryptionScheme { # [doc = " Get the [`SymmetricCipher`] to be used."] pub fn cipher (self) -> SymmetricCipher { match self { Self :: PbeWithMd2AndDesCbc => SymmetricCipher :: DesCbc , Self :: PbeWithMd2AndRc2Cbc => SymmetricCipher :: Rc2Cbc , Self :: PbeWithMd5AndDesCbc => SymmetricCipher :: DesCbc , Self :: PbeWithMd5AndRc2Cbc => SymmetricCipher :: Rc2Cbc , Self :: PbeWithSha1AndDesCbc => SymmetricCipher :: DesCbc , Self :: PbeWithSha1AndRc2Cbc => SymmetricCipher :: Rc2Cbc , } } # [doc = " Get the [`DigestAlgorithm`] to be used."] pub fn digest (self) -> DigestAlgorithm { match self { Self :: PbeWithMd2AndDesCbc => DigestAlgorithm :: Md2 , Self :: PbeWithMd2AndRc2Cbc => DigestAlgorithm :: Md2 , Self :: PbeWithMd5AndDesCbc => DigestAlgorithm :: Md5 , Self :: PbeWithMd5AndRc2Cbc => DigestAlgorithm :: Md5 , Self :: PbeWithSha1AndDesCbc => DigestAlgorithm :: Sha1 , Self :: PbeWithSha1AndRc2Cbc => DigestAlgorithm :: Sha1 , } } # [doc = " Get the [`ObjectIdentifier`] (a.k.a OID) for this algorithm."] pub fn oid (self) -> ObjectIdentifier { match self { Self :: PbeWithMd2AndDesCbc => PBE_WITH_MD2_AND_DES_CBC_OID , Self :: PbeWithMd2AndRc2Cbc => PBE_WITH_MD2_AND_RC2_CBC_OID , Self :: PbeWithMd5AndDesCbc => PBE_WITH_MD5_AND_DES_CBC_OID , Self :: PbeWithMd5AndRc2Cbc => PBE_WITH_MD5_AND_RC2_CBC_OID , Self :: PbeWithSha1AndDesCbc => PBE_WITH_SHA1_AND_DES_CBC_OID , Self :: PbeWithSha1AndRc2Cbc => PBE_WITH_SHA1_AND_RC2_CBC_OID , } } }
};
}
