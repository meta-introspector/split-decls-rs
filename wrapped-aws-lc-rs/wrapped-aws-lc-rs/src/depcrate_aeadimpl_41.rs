// Generated macro for impl_41 (impl)
macro_rules! Depcrate_aeadimpl_41 {
() => {
// Module: crate::aead
// Provides: {"impl_41"}
// Dependencies: {}
impl < N : NonceSequence > OpeningKeyPreparedNonce < '_ , N > { # [doc = " Authenticates and decrypts (“opens”) data in place."] # [doc = ""] # [doc = " See [OpeningKey::open_in_place] for additional API information."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` when ciphertext is invalid. In this case, `in_out` may have been"] # [doc = " overwritten in an unspecified way."] # [inline] # [allow (clippy :: needless_pass_by_value)] pub fn open_in_place < A > (self , aad : Aad < A > , in_out : & mut [u8]) -> Result < & mut [u8] , Unspecified > where A : AsRef < [u8] > , { self . open_within (aad , in_out , 0 ..) } # [doc = " Authenticates and decrypts (“opens”) data in place, with a shift."] # [doc = ""] # [doc = " See [OpeningKey::open_within] for additional API information."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` when ciphertext is invalid. In this case, `in_out` may have been"] # [doc = " overwritten in an unspecified way."] # [inline] # [allow (clippy :: needless_pass_by_value)] pub fn open_within < A > (self , aad : Aad < A > , in_out : & mut [u8] , ciphertext_and_tag : RangeFrom < usize > ,) -> Result < & mut [u8] , Unspecified > where A : AsRef < [u8] > , { self . key . key . open_within (self . nonce , aad . as_ref () , in_out , ciphertext_and_tag) } }
};
}
