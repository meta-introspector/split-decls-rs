// Generated macro for impl_29 (impl)
macro_rules! Depcrate_private_key_infoimpl_29 {
() => {
// Module: crate::private_key_info
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a , Params , Key , PubKey > PrivateKeyInfo < Params , Key , PubKey > where Params : der :: Choice < 'a , Error = der :: Error > + Encode , Key : DecodeValue < 'a , Error = der :: Error > + FixedTag + 'a , Key : EncodeValue , PubKey : DecodeValue < 'a , Error = der :: Error > + FixedTag + 'a , PubKey : BitStringLike , { # [doc = " Encrypt this private key using a symmetric encryption key derived"] # [doc = " from the provided password."] # [doc = ""] # [doc = " Uses the following algorithms for encryption:"] # [doc = " - PBKDF: scrypt with default parameters:"] # [doc = "   - log₂(N): 15"] # [doc = "   - r: 8"] # [doc = "   - p: 1"] # [doc = " - Cipher: AES-256-CBC (best available option for PKCS#5 encryption)"] # [cfg (feature = "encryption")] pub fn encrypt < R : CryptoRng > (& self , rng : & mut R , password : impl AsRef < [u8] > ,) -> Result < SecretDocument > { let der = Zeroizing :: new (self . to_der () ?) ; EncryptedPrivateKeyInfoRef :: encrypt (rng , password , der . as_ref ()) } # [doc = " Encrypt this private key using a symmetric encryption key derived"] # [doc = " from the provided password and [`pbes2::Parameters`]."] # [cfg (feature = "encryption")] pub fn encrypt_with_params (& self , pbes2_params : pbes2 :: Parameters , password : impl AsRef < [u8] > ,) -> Result < SecretDocument > { let der = Zeroizing :: new (self . to_der () ?) ; EncryptedPrivateKeyInfoRef :: encrypt_with (pbes2_params , password , der . as_ref ()) } }
};
}
