// Generated macro for generate_private_key (function)
macro_rules! Depcrate_backend_rsagenerate_private_key {
() => {
// Module: crate::backend::rsa
// Provides: {"generate_private_key"}
// Dependencies: {}
# [pyo3 :: pyfunction] fn generate_private_key (public_exponent : u32 , key_size : u32) -> CryptographyResult < RsaPrivateKey > { let e = openssl :: bn :: BigNum :: from_u32 (public_exponent) ? ; let rsa = openssl :: rsa :: Rsa :: generate_with_e (key_size , & e) ? ; let pkey = openssl :: pkey :: PKey :: from_rsa (rsa) ? ; Ok (RsaPrivateKey { pkey }) }
};
}
