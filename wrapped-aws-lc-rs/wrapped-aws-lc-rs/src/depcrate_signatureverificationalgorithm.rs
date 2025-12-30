// Generated macro for VerificationAlgorithm (trait)
macro_rules! Depcrate_signatureVerificationAlgorithm {
() => {
// Module: crate::signature
// Provides: {"VerificationAlgorithm"}
// Dependencies: {}
# [doc = " A signature verification algorithm."] pub trait VerificationAlgorithm : Debug + Sync + Any + sealed :: Sealed { # [doc = " Verify the signature `signature` of message `msg` with the public key"] # [doc = " `public_key`."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if inputs not verified."] # [cfg (feature = "ring-sig-verify")] # [deprecated (note = "please use `VerificationAlgorithm::verify_sig` instead")] fn verify (& self , public_key : Input < '_ > , msg : Input < '_ > , signature : Input < '_ > ,) -> Result < () , error :: Unspecified > ; # [doc = " Verify the signature `signature` of message `msg` with the public key"] # [doc = " `public_key`."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if inputs not verified."] fn verify_sig (& self , public_key : & [u8] , msg : & [u8] , signature : & [u8] ,) -> Result < () , error :: Unspecified > ; # [doc = " Verify the signature `signature` of `digest` with the `public_key`."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if inputs not verified."] fn verify_digest_sig (& self , public_key : & [u8] , digest : & Digest , signature : & [u8] ,) -> Result < () , error :: Unspecified > ; }
};
}
