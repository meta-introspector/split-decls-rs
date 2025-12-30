// Generated macro for impl_146 (impl)
macro_rules! Depcrate_verifyingimpl_146 {
() => {
// Module: crate::verifying
// Provides: {"impl_146"}
// Dependencies: {}
# [cfg (feature = "sha2")] impl < C > MultipartVerifier < SignatureWithOid < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , SignatureSize < C > : ArraySize , { fn multipart_verify (& self , msg : & [& [u8]] , sig : & SignatureWithOid < C >) -> Result < () > { use digest :: FixedOutput ; match sig . oid () { ECDSA_SHA224_OID => { let mut digest = Sha224 :: default () ; msg . iter () . for_each (| slice | digest . update (slice)) ; self . verify_prehash (& digest . finalize_fixed () , sig . signature ()) } ECDSA_SHA256_OID => { let mut digest = Sha256 :: default () ; msg . iter () . for_each (| slice | digest . update (slice)) ; self . verify_prehash (& digest . finalize_fixed () , sig . signature ()) } ECDSA_SHA384_OID => { let mut digest = Sha384 :: default () ; msg . iter () . for_each (| slice | digest . update (slice)) ; self . verify_prehash (& digest . finalize_fixed () , sig . signature ()) } ECDSA_SHA512_OID => { let mut digest = Sha512 :: default () ; msg . iter () . for_each (| slice | digest . update (slice)) ; self . verify_prehash (& digest . finalize_fixed () , sig . signature ()) } _ => Err (Error :: new ()) , } } }
};
}
