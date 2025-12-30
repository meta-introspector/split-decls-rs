// Generated macro for impl_35 (impl)
macro_rules! Depcrate_backend_aeadimpl_35 {
() => {
// Module: crate::backend::aead
// Provides: {"impl_35"}
// Dependencies: {}
# [cfg (any (CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC))] impl EvpAead { fn new (algorithm : cryptography_openssl :: aead :: AeadType , key : & [u8] , tag_len : usize ,) -> CryptographyResult < EvpAead > { Ok (EvpAead { ctx : cryptography_openssl :: aead :: AeadCtx :: new (algorithm , key) ? , tag_len , }) } fn encrypt_into (& self , _py : pyo3 :: Python < '_ > , plaintext : & [u8] , aad : Option < Aad < '_ > > , nonce : Option < & [u8] > , buf : & mut [u8] ,) -> CryptographyResult < () > { check_length (plaintext) ? ; let ad = if let Some (Aad :: Single (ad)) = & aad { check_length (ad . as_bytes ()) ? ; ad . as_bytes () } else { assert ! (aad . is_none ()) ; b"" } ; self . ctx . encrypt (plaintext , nonce . unwrap_or (b"") , ad , buf) . map_err (CryptographyError :: from) ? ; Ok (()) } fn decrypt_into (& self , _py : pyo3 :: Python < '_ > , ciphertext : & [u8] , aad : Option < Aad < '_ > > , nonce : Option < & [u8] > , buf : & mut [u8] ,) -> CryptographyResult < () > { let ad = if let Some (Aad :: Single (ad)) = & aad { check_length (ad . as_bytes ()) ? ; ad . as_bytes () } else { assert ! (aad . is_none ()) ; b"" } ; self . ctx . decrypt (ciphertext , nonce . unwrap_or (b"") , ad , buf) . map_err (| _ | exceptions :: InvalidTag :: new_err (())) ? ; Ok (()) } }
};
}
