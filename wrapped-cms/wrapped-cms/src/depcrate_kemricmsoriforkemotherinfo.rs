// Generated macro for CmsOriForKemOtherInfo (struct)
macro_rules! Depcrate_kemriCmsOriForKemOtherInfo {
() => {
// Module: crate::kemri
// Provides: {"CmsOriForKemOtherInfo"}
// Dependencies: {}
# [doc = " The `CMSORIforKEMOtherInfo` type is defined in [RFC9629 Section 5]"] # [doc = " ```text"] # [doc = "       CMSORIforKEMOtherInfo ::= SEQUENCE {"] # [doc = "         wrap KeyEncryptionAlgorithmIdentifier,"] # [doc = "         kekLength INTEGER (1..65535),"] # [doc = "         ukm [0] EXPLICIT UserKeyingMaterial OPTIONAL }"] # [doc = " ```"] # [doc = " [RFC9629 Section 5]: https://datatracker.ietf.org/doc/html/rfc9629#section-5"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct CmsOriForKemOtherInfo { pub wrap : AlgorithmIdentifierOwned , pub kek_length : u16 , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , optional = "true")] pub ukm : Option < UserKeyingMaterial > , }
};
}
