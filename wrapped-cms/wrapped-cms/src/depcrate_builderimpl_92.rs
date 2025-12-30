// Generated macro for impl_92 (impl)
macro_rules! Depcrate_builderimpl_92 {
() => {
// Module: crate::builder
// Provides: {"impl_92"}
// Dependencies: {}
impl < P , R > RecipientInfoBuilder for PasswordRecipientInfoBuilder < P , R > where P : PwriEncryptor , R : CryptoRng + ? Sized , { type Rng = R ; # [doc = " Returns the RecipientInfoType"] fn recipient_info_type (& self) -> RecipientInfoType { RecipientInfoType :: Pwri } # [doc = " Returns the `CMSVersion` for this `RecipientInfo`"] fn recipient_info_version (& self) -> CmsVersion { CmsVersion :: V0 } # [doc = " Build a `PasswordRecipientInfoBuilder`. See RFC 5652 § 6.2.1"] fn build_with_rng (& mut self , content_encryption_key : & [u8] , rng : & mut Self :: Rng ,) -> Result < RecipientInfo > { let padded_cek = self . pad_content_encryption_key (content_encryption_key , rng) ? ; let encrypted_key = self . key_encryptor . encrypt_rfc3211 (padded_cek . as_slice () , rng) ? ; let enc_key = OctetString :: new (encrypted_key) ? ; Ok (RecipientInfo :: Pwri (PasswordRecipientInfo { version : self . recipient_info_version () , key_derivation_alg : self . key_derivation_alg . clone () , key_enc_alg : self . key_enc_alg . clone () , enc_key , })) } }
};
}
