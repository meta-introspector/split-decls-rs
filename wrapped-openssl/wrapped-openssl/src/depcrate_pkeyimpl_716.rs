// Generated macro for impl_716 (impl)
macro_rules! Depcrate_pkeyimpl_716 {
() => {
// Module: crate::pkey
// Provides: {"impl_716"}
// Dependencies: {}
impl PKey < Public > { private_key_from_pem ! { # [doc = " Decodes a PEM-encoded SubjectPublicKeyInfo structure."] # [doc = ""] # [doc = " The input should have a header of `-----BEGIN PUBLIC KEY-----`."] # [corresponds (PEM_read_bio_PUBKEY)] public_key_from_pem , # [doc = " Decodes a PEM-encoded SubjectPublicKeyInfo structure."] # [corresponds (PEM_read_bio_PUBKEY)] public_key_from_pem_passphrase , # [doc = " Decodes a PEM-encoded SubjectPublicKeyInfo structure."] # [doc = ""] # [doc = " The callback should fill the password into the provided buffer and return its length."] # [corresponds (PEM_read_bio_PrivateKey)] public_key_from_pem_callback , PKey < Public >, ffi :: PEM_read_bio_PUBKEY } from_der ! { # [doc = " Decodes a DER-encoded SubjectPublicKeyInfo structure."] # [corresponds (d2i_PUBKEY)] public_key_from_der , PKey < Public >, ffi :: d2i_PUBKEY } # [doc = " Creates a public key from its raw byte representation"] # [doc = ""] # [doc = " Algorithm types that support raw public keys are X25519, ED25519, X448 or ED448"] # [corresponds (EVP_PKEY_new_raw_public_key)] # [cfg (any (ossl111 , boringssl , libressl370 , awslc))] pub fn public_key_from_raw_bytes (bytes : & [u8] , key_type : Id ,) -> Result < PKey < Public > , ErrorStack > { unsafe { ffi :: init () ; cvt_p (ffi :: EVP_PKEY_new_raw_public_key (key_type . as_raw () , ptr :: null_mut () , bytes . as_ptr () , bytes . len () ,)) . map (| p | PKey :: from_ptr (p)) } } }
};
}
