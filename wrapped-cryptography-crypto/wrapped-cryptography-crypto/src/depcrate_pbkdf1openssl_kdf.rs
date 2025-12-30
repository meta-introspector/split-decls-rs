// Generated macro for openssl_kdf (function)
macro_rules! Depcrate_pbkdf1openssl_kdf {
() => {
// Module: crate::pbkdf1
// Provides: {"openssl_kdf"}
// Dependencies: {}
# [doc = " This is the OpenSSL KDF that's used in decrypting PEM blocks. It is a"] # [doc = " generalization of PBKDF1."] pub fn openssl_kdf (hash_alg : openssl :: hash :: MessageDigest , password : & [u8] , salt : [u8 ; 8] , length : usize ,) -> Result < Vec < u8 > , openssl :: error :: ErrorStack > { let mut key = Vec :: with_capacity (length) ; while key . len () < length { let mut h = openssl :: hash :: Hasher :: new (hash_alg) ? ; if ! key . is_empty () { h . update (& key [key . len () - hash_alg . size () ..]) ? ; } h . update (password) ? ; h . update (& salt) ? ; let digest = h . finish () ? ; let size = digest . len () . min (length - key . len ()) ; key . extend_from_slice (& digest [.. size]) ; } Ok (key) }
};
}
