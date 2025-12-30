// Generated macro for impl_350 (impl)
macro_rules! Depcrate_ecimpl_350 {
() => {
// Module: crate::ec
// Provides: {"impl_350"}
// Dependencies: {}
impl < T > EcKeyRef < T > where T : HasPrivate , { private_key_to_pem ! { # [doc = " Serializes the private key to a PEM-encoded ECPrivateKey structure."] # [doc = ""] # [doc = " The output will have a header of `-----BEGIN EC PRIVATE KEY-----`."] # [corresponds (PEM_write_bio_ECPrivateKey)] private_key_to_pem , # [doc = " Serializes the private key to a PEM-encoded encrypted ECPrivateKey structure."] # [doc = ""] # [doc = " The output will have a header of `-----BEGIN EC PRIVATE KEY-----`."] # [corresponds (PEM_write_bio_ECPrivateKey)] private_key_to_pem_passphrase , ffi :: PEM_write_bio_ECPrivateKey } to_der ! { # [doc = " Serializes the private key into a DER-encoded ECPrivateKey structure."] # [corresponds (i2d_ECPrivateKey)] private_key_to_der , ffi :: i2d_ECPrivateKey } # [doc = " Returns the private key value."] # [corresponds (EC_KEY_get0_private_key)] pub fn private_key (& self) -> & BigNumRef { unsafe { let ptr = ffi :: EC_KEY_get0_private_key (self . as_ptr ()) ; BigNumRef :: from_const_ptr (ptr) } } }
};
}
