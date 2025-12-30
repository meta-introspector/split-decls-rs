// Generated macro for serialize_pkcs1_public_key (function)
macro_rules! Depcrate_rsaserialize_pkcs1_public_key {
() => {
// Module: crate::rsa
// Provides: {"serialize_pkcs1_public_key"}
// Dependencies: {}
pub fn serialize_pkcs1_public_key (rsa : & openssl :: rsa :: RsaRef < impl openssl :: pkey :: HasPublic > ,) -> KeySerializationResult < Vec < u8 > > { let n_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (rsa . n ()) ? ; let e_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (rsa . e ()) ? ; let key = Pkcs1RsaPublicKey { n : asn1 :: BigUint :: new (& n_bytes) . unwrap () , e : asn1 :: BigUint :: new (& e_bytes) . unwrap () , } ; Ok (asn1 :: write_single (& key) ?) }
};
}
