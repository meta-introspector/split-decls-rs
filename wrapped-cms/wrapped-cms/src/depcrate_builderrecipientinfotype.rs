// Generated macro for RecipientInfoType (enum)
macro_rules! Depcrate_builderRecipientInfoType {
() => {
// Module: crate::builder
// Provides: {"RecipientInfoType"}
// Dependencies: {}
# [doc = " `RecipientInfoBuilder` must be implemented for these 5 recipient info types"] # [doc = " as defined in RFC 5652 § 6:"] # [derive (Clone , Debug , Eq , PartialEq)] pub enum RecipientInfoType { # [doc = " KeyTransRecipientInfo"] Ktri , # [doc = " KeyAgreeRecipientInfo"] Kari , # [doc = " KekRecipientInfo"] Kekri , # [doc = " PasswordRecipientInfo"] Pwri , # [doc = " OtherRecipientInfo"] Ori , }
};
}
