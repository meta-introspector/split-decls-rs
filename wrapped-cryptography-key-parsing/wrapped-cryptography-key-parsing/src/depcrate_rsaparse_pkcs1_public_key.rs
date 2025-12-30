// Generated macro for parse_pkcs1_public_key (function)
macro_rules! Depcrate_rsaparse_pkcs1_public_key {
() => {
// Module: crate::rsa
// Provides: {"parse_pkcs1_public_key"}
// Dependencies: {}
pub fn parse_pkcs1_public_key (data : & [u8] ,) -> KeyParsingResult < openssl :: pkey :: PKey < openssl :: pkey :: Public > > { let k = asn1 :: parse_single :: < Pkcs1RsaPublicKey < '_ > > (data) ? ; let n = openssl :: bn :: BigNum :: from_slice (k . n . as_bytes ()) ? ; let e = openssl :: bn :: BigNum :: from_slice (k . e . as_bytes ()) ? ; let rsa = openssl :: rsa :: Rsa :: from_public_components (n , e) ? ; Ok (openssl :: pkey :: PKey :: from_rsa (rsa) ?) }
};
}
