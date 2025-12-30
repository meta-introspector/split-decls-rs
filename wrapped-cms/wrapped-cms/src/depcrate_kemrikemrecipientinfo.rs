// Generated macro for KemRecipientInfo (struct)
macro_rules! Depcrate_kemriKemRecipientInfo {
() => {
// Module: crate::kemri
// Provides: {"KemRecipientInfo"}
// Dependencies: {}
# [doc = " The `KEMRecipientInfo` type is defined in [RFC9629 Section 3]"] # [doc = " ```text"] # [doc = "   KEMRecipientInfo ::= SEQUENCE {"] # [doc = "     version CMSVersion,  -- always set to 0"] # [doc = "     rid RecipientIdentifier,"] # [doc = "     kem KEMAlgorithmIdentifier,"] # [doc = "     kemct OCTET STRING,"] # [doc = "     kdf KeyDerivationAlgorithmIdentifier,"] # [doc = "     kekLength INTEGER (1..65535),"] # [doc = "     ukm [0] EXPLICIT UserKeyingMaterial OPTIONAL,"] # [doc = "     wrap KeyEncryptionAlgorithmIdentifier,"] # [doc = "     encryptedKey EncryptedKey }"] # [doc = " ```"] # [doc = " [RFC9629 Section 3]: https://datatracker.ietf.org/doc/html/rfc9629#section-3"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct KemRecipientInfo { pub version : CmsVersion , pub rid : RecipientIdentifier , pub kem : AlgorithmIdentifierOwned , pub kem_ct : OctetString , pub kdf : AlgorithmIdentifierOwned , pub kek_length : u16 , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , optional = "true")] pub ukm : Option < UserKeyingMaterial > , pub wrap : AlgorithmIdentifierOwned , pub encrypted_key : EncryptedKey , }
};
}
