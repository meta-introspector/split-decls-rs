// Generated macro for impl_42 (impl)
macro_rules! Depcrate_aeadimpl_42 {
() => {
// Module: crate::aead
// Provides: {"impl_42"}
// Dependencies: {}
impl < N : NonceSequence > SealingKeyPreparedNonce < '_ , N > { # [doc = " Encrypts and signs (“seals”) data in place, appending the tag to the"] # [doc = " resulting ciphertext."] # [doc = ""] # [doc = " See [SealingKey::seal_in_place_append_tag] for additional API information."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` when `nonce_sequence` cannot be advanced."] # [inline] # [allow (clippy :: needless_pass_by_value)] pub fn seal_in_place_append_tag < A , InOut > (self , aad : Aad < A > , in_out : & mut InOut ,) -> Result < () , Unspecified > where A : AsRef < [u8] > , InOut : AsMut < [u8] > + for < 'in_out > Extend < & 'in_out u8 > , { self . key . key . seal_in_place_append_tag (Some (self . nonce) , aad . as_ref () , in_out) . map (| _ | ()) } # [doc = " Encrypts and signs (“seals”) data in place."] # [doc = ""] # [doc = " See [`SealingKey::seal_in_place_separate_tag`] for additional API information."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` when `nonce_sequence` cannot be advanced."] # [inline] # [allow (clippy :: needless_pass_by_value)] pub fn seal_in_place_separate_tag < A > (self , aad : Aad < A > , in_out : & mut [u8] ,) -> Result < Tag , Unspecified > where A : AsRef < [u8] > , { self . key . key . seal_in_place_separate_tag (Some (self . nonce) , aad . as_ref () , in_out) . map (| (_ , tag) | tag) } }
};
}
