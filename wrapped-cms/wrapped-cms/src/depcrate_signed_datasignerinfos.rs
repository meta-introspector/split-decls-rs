// Generated macro for SignerInfos (struct)
macro_rules! Depcrate_signed_dataSignerInfos {
() => {
// Module: crate::signed_data
// Provides: {"SignerInfos"}
// Dependencies: {}
# [doc = " The `SignerInfos` type is defined in [RFC 5652 Section 5.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   SignerInfos ::= SET OF SignerInfo"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 5.1]: https://www.rfc-editor.org/rfc/rfc5652#section-5.1"] # [derive (Clone , Eq , PartialEq , Debug)] pub struct SignerInfos (pub SetOfVec < SignerInfo >) ;
};
}
