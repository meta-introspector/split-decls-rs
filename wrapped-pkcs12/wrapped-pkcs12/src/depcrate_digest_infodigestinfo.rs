// Generated macro for DigestInfo (struct)
macro_rules! Depcrate_digest_infoDigestInfo {
() => {
// Module: crate::digest_info
// Provides: {"DigestInfo"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " DigestInfo ::= SEQUENCE {"] # [doc = " digestAlgorithm DigestAlgorithmIdentifier,"] # [doc = " digest Digest }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence , ValueOrd)] pub struct DigestInfo { # [doc = " the algorithm."] pub algorithm : AlgorithmIdentifierOwned , # [doc = " the digest"] pub digest : OctetString , }
};
}
