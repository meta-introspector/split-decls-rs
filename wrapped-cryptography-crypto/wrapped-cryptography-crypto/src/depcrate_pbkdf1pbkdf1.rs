// Generated macro for pbkdf1 (function)
macro_rules! Depcrate_pbkdf1pbkdf1 {
() => {
// Module: crate::pbkdf1
// Provides: {"pbkdf1"}
// Dependencies: {}
# [doc = " PBKDF1 as defined in RFC 2898 for PKCS#5 v1.5 PBE algorithms"] pub fn pbkdf1 (hash_alg : openssl :: hash :: MessageDigest , password : & [u8] , salt : [u8 ; 8] , iterations : u64 , length : usize ,) -> Result < Vec < u8 > , openssl :: error :: ErrorStack > { if length > hash_alg . size () || iterations == 0 { return Err (openssl :: error :: ErrorStack :: get ()) ; } let mut h = openssl :: hash :: Hasher :: new (hash_alg) ? ; h . update (password) ? ; h . update (& salt) ? ; let mut t = h . finish () ? ; for _ in 1 .. iterations { let mut h = openssl :: hash :: Hasher :: new (hash_alg) ? ; h . update (& t) ? ; t = h . finish () ? ; } Ok (t [.. length] . to_vec ()) }
};
}
