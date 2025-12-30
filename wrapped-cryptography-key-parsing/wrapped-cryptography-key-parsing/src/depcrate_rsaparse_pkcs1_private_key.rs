// Generated macro for parse_pkcs1_private_key (function)
macro_rules! Depcrate_rsaparse_pkcs1_private_key {
() => {
// Module: crate::rsa
// Provides: {"parse_pkcs1_private_key"}
// Dependencies: {}
pub fn parse_pkcs1_private_key (data : & [u8] ,) -> KeyParsingResult < openssl :: pkey :: PKey < openssl :: pkey :: Private > > { let rsa_private_key = asn1 :: parse_single :: < RsaPrivateKey < '_ > > (data) ? ; if rsa_private_key . version != 0 || rsa_private_key . other_prime_infos . is_some () { return Err (KeyParsingError :: InvalidKey) ; } let n = openssl :: bn :: BigNum :: from_slice (rsa_private_key . n . as_bytes ()) ? ; let e = openssl :: bn :: BigNum :: from_slice (rsa_private_key . e . as_bytes ()) ? ; let d = openssl :: bn :: BigNum :: from_slice (rsa_private_key . d . as_bytes ()) ? ; let p = openssl :: bn :: BigNum :: from_slice (rsa_private_key . p . as_bytes ()) ? ; let q = openssl :: bn :: BigNum :: from_slice (rsa_private_key . q . as_bytes ()) ? ; let dmp1 = openssl :: bn :: BigNum :: from_slice (rsa_private_key . dmp1 . as_bytes ()) ? ; let dmq1 = openssl :: bn :: BigNum :: from_slice (rsa_private_key . dmq1 . as_bytes ()) ? ; let iqmp = openssl :: bn :: BigNum :: from_slice (rsa_private_key . iqmp . as_bytes ()) ? ; let rsa_key = openssl :: rsa :: Rsa :: from_private_components (n , e , d , p , q , dmp1 , dmq1 , iqmp) ? ; Ok (openssl :: pkey :: PKey :: from_rsa (rsa_key) ?) }
};
}
