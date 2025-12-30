// Generated macro for RecipientInfoBuilder (trait)
macro_rules! Depcrate_builderRecipientInfoBuilder {
() => {
// Module: crate::builder
// Provides: {"RecipientInfoBuilder"}
// Dependencies: {}
# [doc = " Trait for builders of a `RecipientInfo`. RFC 5652 § 6 defines 5 different `RecipientInfo`"] # [doc = " formats. All implementations must implement this trait."] pub trait RecipientInfoBuilder { # [doc = " Associated Rng type"] type Rng : CryptoRng + ? Sized ; # [doc = " Return the recipient info type"] fn recipient_info_type (& self) -> RecipientInfoType ; # [doc = " Return the recipient info version"] fn recipient_info_version (& self) -> CmsVersion ; # [doc = " Encrypt the `content_encryption_key` using a method, that is specific for the implementing"] # [doc = " builder type. Finally return a `RecipientInfo`."] fn build_with_rng (& mut self , content_encryption_key : & [u8] , rng : & mut Self :: Rng ,) -> Result < RecipientInfo > ; }
};
}
