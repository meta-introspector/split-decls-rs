// Generated macro for impl_310 (impl)
macro_rules! Depcrate_dsaimpl_310 {
() => {
// Module: crate::dsa
// Provides: {"impl_310"}
// Dependencies: {}
impl < T > DsaRef < T > where T : HasPrivate , { private_key_to_pem ! { # [doc = " Serializes the private key to a PEM-encoded DSAPrivateKey structure."] # [doc = ""] # [doc = " The output will have a header of `-----BEGIN DSA PRIVATE KEY-----`."] # [corresponds (PEM_write_bio_DSAPrivateKey)] private_key_to_pem , # [doc = " Serializes the private key to a PEM-encoded encrypted DSAPrivateKey structure."] # [doc = ""] # [doc = " The output will have a header of `-----BEGIN DSA PRIVATE KEY-----`."] # [corresponds (PEM_write_bio_DSAPrivateKey)] private_key_to_pem_passphrase , ffi :: PEM_write_bio_DSAPrivateKey } to_der ! { # [doc = " Serializes the private_key to a DER-encoded `DSAPrivateKey` structure."] # [corresponds (i2d_DSAPrivateKey)] private_key_to_der , ffi :: i2d_DSAPrivateKey } # [doc = " Returns a reference to the private key component of `self`."] # [corresponds (DSA_get0_key)] pub fn priv_key (& self) -> & BigNumRef { unsafe { let mut priv_key = ptr :: null () ; DSA_get0_key (self . as_ptr () , ptr :: null_mut () , & mut priv_key) ; BigNumRef :: from_const_ptr (priv_key) } } }
};
}
