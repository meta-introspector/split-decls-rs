// Generated macro for serialize_pkcs1_private_key (function)
macro_rules! Depcrate_dsaserialize_pkcs1_private_key {
() => {
// Module: crate::dsa
// Provides: {"serialize_pkcs1_private_key"}
// Dependencies: {}
pub fn serialize_pkcs1_private_key (dsa : & openssl :: dsa :: DsaRef < openssl :: pkey :: Private > ,) -> KeySerializationResult < Vec < u8 > > { let p_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (dsa . p ()) ? ; let q_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (dsa . q ()) ? ; let g_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (dsa . g ()) ? ; let pub_key_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (dsa . pub_key ()) ? ; let priv_key_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (dsa . priv_key ()) ? ; let key = DsaPrivateKey { version : 0 , p : asn1 :: BigUint :: new (& p_bytes) . unwrap () , q : asn1 :: BigUint :: new (& q_bytes) . unwrap () , g : asn1 :: BigUint :: new (& g_bytes) . unwrap () , pub_key : asn1 :: BigUint :: new (& pub_key_bytes) . unwrap () , priv_key : asn1 :: BigUint :: new (& priv_key_bytes) . unwrap () , } ; Ok (asn1 :: write_single (& key) ?) }
};
}
