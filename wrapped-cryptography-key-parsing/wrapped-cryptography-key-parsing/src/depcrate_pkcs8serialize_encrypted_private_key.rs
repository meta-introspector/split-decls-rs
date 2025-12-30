// Generated macro for serialize_encrypted_private_key (function)
macro_rules! Depcrate_pkcs8serialize_encrypted_private_key {
() => {
// Module: crate::pkcs8
// Provides: {"serialize_encrypted_private_key"}
// Dependencies: {}
pub fn serialize_encrypted_private_key (pkey : & openssl :: pkey :: PKeyRef < openssl :: pkey :: Private > , password : & [u8] ,) -> crate :: KeySerializationResult < Vec < u8 > > { let plaintext_der = serialize_private_key (pkey) ? ; let e = pbe :: EncryptionAlgorithm :: PBESv2SHA256AndAES256CBC ; let mut salt = [0u8 ; 16] ; let mut iv = [0u8 ; 16] ; cryptography_openssl :: rand :: rand_bytes (& mut salt) ? ; cryptography_openssl :: rand :: rand_bytes (& mut iv) ? ; let encrypted_data = e . encrypt (password , KDF_ITERATION_COUNT , & salt , & iv , & plaintext_der) ? ; let encryption_alg = e . algorithm_identifier (KDF_ITERATION_COUNT , & salt , & iv) ; let epki = cryptography_x509 :: pkcs8 :: EncryptedPrivateKeyInfo { encryption_algorithm : encryption_alg , encrypted_data : & encrypted_data , } ; Ok (asn1 :: write_single (& epki) ?) }
};
}
