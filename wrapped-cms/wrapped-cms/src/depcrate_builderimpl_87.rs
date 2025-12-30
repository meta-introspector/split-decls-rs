// Generated macro for impl_87 (impl)
macro_rules! Depcrate_builderimpl_87 {
() => {
// Module: crate::builder
// Provides: {"impl_87"}
// Dependencies: {}
impl < R : ? Sized > RecipientInfoBuilder for KekRecipientInfoBuilder < R > where R : CryptoRng , { type Rng = R ; # [doc = " Returns the RecipientInfoType"] fn recipient_info_type (& self) -> RecipientInfoType { RecipientInfoType :: Kekri } # [doc = " Returns the `CMSVersion` for this `RecipientInfo`"] fn recipient_info_version (& self) -> CmsVersion { CmsVersion :: V4 } # [doc = " Build a `KekRecipientInfoBuilder`. See RFC 5652 § 6.2.1"] fn build_with_rng (& mut self , _content_encryption_key : & [u8] , _rng : & mut Self :: Rng ,) -> Result < RecipientInfo > { Err (Error :: Builder (String :: from ("Building KekRecipientInfo is not implemented, yet." ,))) } }
};
}
