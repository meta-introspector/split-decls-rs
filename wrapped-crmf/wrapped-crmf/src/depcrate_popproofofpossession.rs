// Generated macro for ProofOfPossession (enum)
macro_rules! Depcrate_popProofOfPossession {
() => {
// Module: crate::pop
// Provides: {"ProofOfPossession"}
// Dependencies: {}
# [doc = " The `ProofOfPossession` type is defined in [RFC 4211 Section 4]."] # [doc = ""] # [doc = " ```text"] # [doc = "   ProofOfPossession ::= CHOICE {"] # [doc = "       raVerified        [0] NULL,"] # [doc = "       -- used if the RA has already verified that the requester is in"] # [doc = "       -- possession of the private key"] # [doc = "       signature         [1] POPOSigningKey,"] # [doc = "       keyEncipherment   [2] POPOPrivKey,"] # [doc = "       keyAgreement      [3] POPOPrivKey }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 4]: https://www.rfc-editor.org/rfc/rfc4211#section-4"] # [derive (Clone , Debug , PartialEq , Eq , Choice)] # [allow (missing_docs)] pub enum ProofOfPossession { # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT" , constructed = "false")] RaVerified (Null) , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , constructed = "true")] Signature (Box < PopoSigningKey >) , # [asn1 (context_specific = "2" , tag_mode = "EXPLICIT" , constructed = "true")] KeyEncipherment (POPOPrivKey) , # [asn1 (context_specific = "3" , tag_mode = "EXPLICIT" , constructed = "true")] KeyAgreement (POPOPrivKey) , }
};
}
