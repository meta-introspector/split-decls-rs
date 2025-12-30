// Generated macro for impl_84 (impl)
macro_rules! Depcrate_builderimpl_84 {
() => {
// Module: crate::builder
// Provides: {"impl_84"}
// Dependencies: {}
impl < R : ? Sized > RecipientInfoBuilder for KeyTransRecipientInfoBuilder < R > where R : CryptoRng , { type Rng = R ; fn recipient_info_type (& self) -> RecipientInfoType { RecipientInfoType :: Ktri } fn recipient_info_version (& self) -> CmsVersion { match self . rid { RecipientIdentifier :: IssuerAndSerialNumber (_) => CmsVersion :: V0 , RecipientIdentifier :: SubjectKeyIdentifier (_) => CmsVersion :: V2 , } } # [doc = " Build a `KeyTransRecipientInfo`. See RFC 5652 § 6.2.1"] # [doc = " `content_encryption_key` will be encrypted with the recipient's public key."] fn build_with_rng (& mut self , content_encryption_key : & [u8] , rng : & mut Self :: Rng ,) -> Result < RecipientInfo > { let (encrypted_key , key_enc_alg) = match & self . key_encryption_info { KeyEncryptionInfo :: Rsa (recipient_public_key) => (recipient_public_key . encrypt (rng , Pkcs1v15Encrypt , content_encryption_key) . map_err (| _ | Error :: Builder (String :: from ("Could not encrypt key"))) ? , AlgorithmIdentifierOwned { oid : const_oid :: db :: rfc5912 :: RSA_ENCRYPTION , parameters : Some (Any :: from (Null)) , } ,) , } ; let enc_key = EncryptedKey :: new (encrypted_key) ? ; Ok (RecipientInfo :: Ktri (KeyTransRecipientInfo { version : self . recipient_info_version () , rid : self . rid . clone () , key_enc_alg , enc_key , })) } }
};
}
