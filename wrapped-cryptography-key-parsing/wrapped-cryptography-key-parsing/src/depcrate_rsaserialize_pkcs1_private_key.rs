// Generated macro for serialize_pkcs1_private_key (function)
macro_rules! Depcrate_rsaserialize_pkcs1_private_key {
() => {
// Module: crate::rsa
// Provides: {"serialize_pkcs1_private_key"}
// Dependencies: {}
pub fn serialize_pkcs1_private_key (rsa : & openssl :: rsa :: RsaRef < openssl :: pkey :: Private > ,) -> KeySerializationResult < Vec < u8 > > { let n_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (rsa . n ()) ? ; let e_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (rsa . e ()) ? ; let d_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (rsa . d ()) ? ; let p_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (rsa . p () . unwrap ()) ? ; let q_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (rsa . q () . unwrap ()) ? ; let dmp1_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (rsa . dmp1 () . unwrap ()) ? ; let dmq1_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (rsa . dmq1 () . unwrap ()) ? ; let iqmp_bytes = cryptography_openssl :: utils :: bn_to_big_endian_bytes (rsa . iqmp () . unwrap ()) ? ; let key = RsaPrivateKey { version : 0 , n : asn1 :: BigUint :: new (& n_bytes) . unwrap () , e : asn1 :: BigUint :: new (& e_bytes) . unwrap () , d : asn1 :: BigUint :: new (& d_bytes) . unwrap () , p : asn1 :: BigUint :: new (& p_bytes) . unwrap () , q : asn1 :: BigUint :: new (& q_bytes) . unwrap () , dmp1 : asn1 :: BigUint :: new (& dmp1_bytes) . unwrap () , dmq1 : asn1 :: BigUint :: new (& dmq1_bytes) . unwrap () , iqmp : asn1 :: BigUint :: new (& iqmp_bytes) . unwrap () , other_prime_infos : None , } ; Ok (asn1 :: write_single (& key) ?) }
};
}
