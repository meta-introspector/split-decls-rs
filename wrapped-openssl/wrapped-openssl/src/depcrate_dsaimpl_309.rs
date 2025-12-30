// Generated macro for impl_309 (impl)
macro_rules! Depcrate_dsaimpl_309 {
() => {
// Module: crate::dsa
// Provides: {"impl_309"}
// Dependencies: {}
impl < T > DsaRef < T > where T : HasPublic , { to_pem ! { # [doc = " Serializes the public key into a PEM-encoded SubjectPublicKeyInfo structure."] # [doc = ""] # [doc = " The output will have a header of `-----BEGIN PUBLIC KEY-----`."] # [corresponds (PEM_write_bio_DSA_PUBKEY)] public_key_to_pem , ffi :: PEM_write_bio_DSA_PUBKEY } to_der ! { # [doc = " Serializes the public key into a DER-encoded SubjectPublicKeyInfo structure."] # [corresponds (i2d_DSA_PUBKEY)] public_key_to_der , ffi :: i2d_DSA_PUBKEY } # [doc = " Returns a reference to the public key component of `self`."] # [corresponds (DSA_get0_key)] pub fn pub_key (& self) -> & BigNumRef { unsafe { let mut pub_key = ptr :: null () ; DSA_get0_key (self . as_ptr () , & mut pub_key , ptr :: null_mut ()) ; BigNumRef :: from_const_ptr (pub_key) } } }
};
}
