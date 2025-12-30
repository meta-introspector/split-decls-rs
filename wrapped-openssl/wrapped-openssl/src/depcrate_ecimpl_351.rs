// Generated macro for impl_351 (impl)
macro_rules! Depcrate_ecimpl_351 {
() => {
// Module: crate::ec
// Provides: {"impl_351"}
// Dependencies: {}
impl < T > EcKeyRef < T > where T : HasPublic , { # [doc = " Returns the public key."] # [corresponds (EC_KEY_get0_public_key)] pub fn public_key (& self) -> & EcPointRef { unsafe { let ptr = ffi :: EC_KEY_get0_public_key (self . as_ptr ()) ; EcPointRef :: from_const_ptr (ptr) } } to_pem ! { # [doc = " Serializes the public key into a PEM-encoded SubjectPublicKeyInfo structure."] # [doc = ""] # [doc = " The output will have a header of `-----BEGIN PUBLIC KEY-----`."] # [corresponds (PEM_write_bio_EC_PUBKEY)] public_key_to_pem , ffi :: PEM_write_bio_EC_PUBKEY } to_der ! { # [doc = " Serializes the public key into a DER-encoded SubjectPublicKeyInfo structure."] # [corresponds (i2d_EC_PUBKEY)] public_key_to_der , ffi :: i2d_EC_PUBKEY } }
};
}
