// Generated macro for RecipientInfos (struct)
macro_rules! Depcrate_enveloped_dataRecipientInfos {
() => {
// Module: crate::enveloped_data
// Provides: {"RecipientInfos"}
// Dependencies: {}
# [doc = " The `RecipientInfos` type is defined in [RFC 5652 Section 6.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   RecipientInfos ::= SET SIZE (1..MAX) OF RecipientInfo"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.1]: https://www.rfc-editor.org/rfc/rfc5652#section-6.1"] # [derive (Clone , Debug , Default , PartialEq , Eq)] pub struct RecipientInfos (pub SetOfVec < RecipientInfo >) ;
};
}
