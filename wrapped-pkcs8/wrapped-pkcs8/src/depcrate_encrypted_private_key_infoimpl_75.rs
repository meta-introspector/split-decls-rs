// Generated macro for impl_75 (impl)
macro_rules! Depcrate_encrypted_private_key_infoimpl_75 {
() => {
// Module: crate::encrypted_private_key_info
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'a , Data > EncryptedPrivateKeyInfo < Data > where Data : DecodeValue < 'a , Error = der :: Error > + EncodeValue + FixedTag + 'a , Data : AsRef < [u8] > , { # [doc = " Attempt to decrypt this encrypted private key using the provided"] # [doc = " password to derive an encryption key."] # [cfg (feature = "encryption")] pub fn decrypt (& self , password : impl AsRef < [u8] >) -> Result < SecretDocument > { Ok (self . encryption_algorithm . decrypt (password , self . encrypted_data . as_ref ()) ? . try_into () ?) } # [doc = " Encrypt the given ASN.1 DER document using a symmetric encryption key"] # [doc = " derived from the provided password."] # [cfg (feature = "encryption")] pub (crate) fn encrypt < R : CryptoRng > (rng : & mut R , password : impl AsRef < [u8] > , doc : & [u8] ,) -> Result < SecretDocument > { let pbes2_params = pbes2 :: Parameters :: recommended (rng) ; EncryptedPrivateKeyInfoOwned :: encrypt_with (pbes2_params , password , doc) } # [doc = " Encrypt this private key using a symmetric encryption key derived"] # [doc = " from the provided password and [`pbes2::Parameters`]."] # [cfg (feature = "encryption")] pub (crate) fn encrypt_with (pbes2_params : pbes2 :: Parameters , password : impl AsRef < [u8] > , doc : & [u8] ,) -> Result < SecretDocument > { let encrypted_data = pbes2_params . encrypt (password , doc) ? ; let encrypted_data = OctetStringRef :: new (& encrypted_data) ? ; EncryptedPrivateKeyInfo { encryption_algorithm : pbes2_params . into () , encrypted_data , } . try_into () } }
};
}
