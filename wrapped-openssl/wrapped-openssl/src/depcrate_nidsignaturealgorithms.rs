// Generated macro for SignatureAlgorithms (struct)
macro_rules! Depcrate_nidSignatureAlgorithms {
() => {
// Module: crate::nid
// Provides: {"SignatureAlgorithms"}
// Dependencies: {}
# [doc = " The digest and public-key algorithms associated with a signature."] pub struct SignatureAlgorithms { # [doc = " The signature's digest."] # [doc = ""] # [doc = " If the signature does not specify a digest, this will be `NID::UNDEF`."] pub digest : Nid , # [doc = " The signature's public-key."] pub pkey : Nid , }
};
}
