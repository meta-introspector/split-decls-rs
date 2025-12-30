// Generated macro for impl_140 (impl)
macro_rules! Depcrate_verifyimpl_140 {
() => {
// Module: crate::verify
// Provides: {"impl_140"}
// Dependencies: {}
impl SignatureVerificationAlgorithm for RsaPssSha256Verify { fn public_key_alg_id (& self) -> AlgorithmIdentifier { alg_id :: RSA_ENCRYPTION } fn signature_alg_id (& self) -> AlgorithmIdentifier { alg_id :: RSA_PSS_SHA256 } fn verify_signature (& self , public_key : & [u8] , message : & [u8] , signature : & [u8] ,) -> Result < () , InvalidSignature > { let public_key = decode_spki_spk (public_key) ? ; let signature = pss :: Signature :: try_from (signature) . map_err (| _ | InvalidSignature) ? ; pss :: VerifyingKey :: < sha2 :: Sha256 > :: new (public_key) . verify (message , & signature) . map_err (| _ | InvalidSignature) } }
};
}
