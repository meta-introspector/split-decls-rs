// Generated macro for VerificationAlgorithm (trait)
macro_rules! Depcrate_signatureVerificationAlgorithm {
() => {
// Module: crate::signature
// Provides: {"VerificationAlgorithm"}
// Dependencies: {}
# [doc = " A signature verification algorithm."] pub trait VerificationAlgorithm : core :: fmt :: Debug + Sync { # [doc = " Verify the signature `signature` of message `msg` with the public key"] # [doc = " `public_key`."] # [doc (hidden)] fn verify_ (& self , public_key : untrusted :: Input , msg : untrusted :: Input , signature : untrusted :: Input , _ : sealed :: Arg ,) -> Result < () , error :: Unspecified > ; # [deprecated (note = "Internal API not intended for external use.")] # [doc (hidden)] fn verify (& self , public_key : untrusted :: Input , msg : untrusted :: Input , signature : untrusted :: Input ,) -> Result < () , error :: Unspecified > { self . verify_ (public_key , msg , signature , sealed :: Arg) } }
};
}
