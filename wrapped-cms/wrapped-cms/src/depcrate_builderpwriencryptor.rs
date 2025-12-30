// Generated macro for PwriEncryptor (trait)
macro_rules! Depcrate_builderPwriEncryptor {
() => {
// Module: crate::builder
// Provides: {"PwriEncryptor"}
// Dependencies: {}
# [doc = " Trait used for encrypting the content-encryption key for PasswordRecipientInfo."] # [doc = " This trait must be implemented by a user and which allows for greater flexibility"] # [doc = " in choosing key derivation and encryption algorithms. Note, that method"] # [doc = " `encrypt_rfc3211()` must follow RFC 3211 and encrypt the key twice."] pub trait PwriEncryptor { # [doc = " Block length of the encryption algorithm."] const BLOCK_LENGTH_BITS : usize ; # [doc = " Returns the algorithm identifier of the used key derivation algorithm,"] # [doc = " which is used to derive an encryption key from the secret/password"] # [doc = " shared with the recipient. Includes eventual parameters (e.g. the used iv)."] fn key_derivation_algorithm (& self) -> Result < Option < AlgorithmIdentifierOwned > > ; # [doc = " Returns the algorithm identifier of the used encryption algorithm"] # [doc = " including eventual parameters (e.g. the used iv)."] fn key_encryption_algorithm (& self) -> Result < AlgorithmIdentifierOwned > ; # [doc = " Encrypt the padded content-encryption key twice following RFC 3211, § 2.3.1"] fn encrypt_rfc3211 < R : CryptoRng + ? Sized > (& mut self , padded_content_encryption_key : & [u8] , rng : & mut R ,) -> Result < Vec < u8 > > ; }
};
}
