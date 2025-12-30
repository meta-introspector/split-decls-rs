// Generated macro for impl_95 (impl)
macro_rules! Depcrate_builderimpl_95 {
() => {
// Module: crate::builder
// Provides: {"impl_95"}
// Dependencies: {}
impl < R > RecipientInfoBuilder for OtherRecipientInfoBuilder < R > where R : CryptoRng + ? Sized , { type Rng = R ; # [doc = " Returns the RecipientInfoType"] fn recipient_info_type (& self) -> RecipientInfoType { RecipientInfoType :: Ori } # [doc = " Returns the `CMSVersion` for this `RecipientInfo`"] fn recipient_info_version (& self) -> CmsVersion { panic ! ("Ori has no CMSVersion") } # [doc = " Build a `OtherRecipientInfoBuilder`. See RFC 5652 § 6.2.1"] fn build_with_rng (& mut self , _content_encryption_key : & [u8] , _rng : & mut Self :: Rng ,) -> Result < RecipientInfo > { panic ! ("Ori has no common build method.") } }
};
}
