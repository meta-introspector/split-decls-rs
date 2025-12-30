// Generated macro for impl_191 (impl)
macro_rules! Depcrate_hpkeimpl_191 {
() => {
// Module: crate::hpke
// Provides: {"impl_191"}
// Dependencies: {}
impl Params { # [doc = " New `Params` from KEM, KDF, and AEAD enums."] pub fn new (kem : Kem , _kdf : Kdf , aead : Aead) -> Self { unsafe { Self { kem : kem . as_ffi_ptr () , kdf : bssl_sys :: EVP_hpke_hkdf_sha256 () , aead : aead . as_ffi_ptr () , } } } # [doc = " New `Params` from KEM, KDF, and AEAD IDs as detailed in RFC 9180."] pub fn new_from_rfc_ids (kem_id : u16 , kdf_id : u16 , aead_id : u16) -> Option < Self > { let kem = Kem :: from_rfc_id (kem_id) ? ; let kdf = Kdf :: HkdfSha256 ; let aead = Aead :: from_rfc_id (aead_id) ? ; if kdf_id != kdf as u16 { return None ; } Some (Self :: new (kem , kdf , aead)) } }
};
}
