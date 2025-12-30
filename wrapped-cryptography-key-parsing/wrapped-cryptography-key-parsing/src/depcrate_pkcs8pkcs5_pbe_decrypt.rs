// Generated macro for pkcs5_pbe_decrypt (function)
macro_rules! Depcrate_pkcs8pkcs5_pbe_decrypt {
() => {
// Module: crate::pkcs8
// Provides: {"pkcs5_pbe_decrypt"}
// Dependencies: {}
fn pkcs5_pbe_decrypt (data : & [u8] , password : & [u8] , cipher : openssl :: symm :: Cipher , hash : openssl :: hash :: MessageDigest , params : & PbeParams ,) -> KeyParsingResult < Vec < u8 > > { let key_iv_len = cipher . key_len () + cipher . iv_len () . unwrap () ; let key_iv = cryptography_crypto :: pbkdf1 :: pbkdf1 (hash , password , params . salt , params . iterations , key_iv_len ,) ? ; let key = & key_iv [.. cipher . key_len ()] ; let iv = & key_iv [cipher . key_len () ..] ; openssl :: symm :: decrypt (cipher , key , Some (iv) , data) . map_err (| _ | KeyParsingError :: IncorrectPassword) }
};
}
