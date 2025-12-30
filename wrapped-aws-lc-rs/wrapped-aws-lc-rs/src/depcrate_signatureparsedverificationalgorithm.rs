// Generated macro for ParsedVerificationAlgorithm (trait)
macro_rules! Depcrate_signatureParsedVerificationAlgorithm {
() => {
// Module: crate::signature
// Provides: {"ParsedVerificationAlgorithm"}
// Dependencies: {}
pub (crate) trait ParsedVerificationAlgorithm : Debug + Sync { fn parsed_verify_sig (& self , public_key : & ParsedPublicKey , msg : & [u8] , signature : & [u8] ,) -> Result < () , error :: Unspecified > ; fn parsed_verify_digest_sig (& self , public_key : & ParsedPublicKey , digest : & Digest , signature : & [u8] ,) -> Result < () , error :: Unspecified > ; }
};
}
