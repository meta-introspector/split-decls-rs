// Generated macro for RecipientInfo (enum)
macro_rules! Depcrate_enveloped_dataRecipientInfo {
() => {
// Module: crate::enveloped_data
// Provides: {"RecipientInfo"}
// Dependencies: {}
# [doc = " The `RecipientInfo` type is defined in [RFC 5652 Section 6.2]."] # [doc = ""] # [doc = " ```text"] # [doc = "   RecipientInfo ::= CHOICE {"] # [doc = "       ktri           KeyTransRecipientInfo,"] # [doc = "       ...,"] # [doc = "       [[3: kari  [1] KeyAgreeRecipientInfo ]],"] # [doc = "       [[4: kekri [2] KEKRecipientInfo]],"] # [doc = "       [[5: pwri  [3] PasswordRecipientInfo,"] # [doc = "            ori   [4] OtherRecipientInfo ]] }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5652 Section 6.2]: https://www.rfc-editor.org/rfc/rfc5652#section-6.2"] # [derive (Clone , Debug , Eq , PartialEq , Choice)] # [allow (missing_docs)] pub enum RecipientInfo { Ktri (KeyTransRecipientInfo) , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , constructed = "true")] Kari (KeyAgreeRecipientInfo) , # [asn1 (context_specific = "2" , tag_mode = "IMPLICIT" , constructed = "true")] Kekri (KekRecipientInfo) , # [asn1 (context_specific = "3" , tag_mode = "IMPLICIT" , constructed = "true")] Pwri (PasswordRecipientInfo) , # [asn1 (context_specific = "4" , tag_mode = "IMPLICIT" , constructed = "true")] Ori (OtherRecipientInfo) , }
};
}
