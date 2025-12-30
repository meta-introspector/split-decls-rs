// Generated macro for pkcs12_pbe_decrypt (function)
macro_rules! Depcrate_pkcs8pkcs12_pbe_decrypt {
() => {
// Module: crate::pkcs8
// Provides: {"pkcs12_pbe_decrypt"}
// Dependencies: {}
fn pkcs12_pbe_decrypt (data : & [u8] , password : & [u8] , cipher : openssl :: symm :: Cipher , hash : openssl :: hash :: MessageDigest , params : & Pkcs12PbeParams < '_ > ,) -> KeyParsingResult < Vec < u8 > > { let Ok (password) = std :: str :: from_utf8 (password) else { return Err (KeyParsingError :: IncorrectPassword) ; } ; let key = cryptography_crypto :: pkcs12 :: kdf (password , params . salt , cryptography_crypto :: pkcs12 :: KDF_ENCRYPTION_KEY_ID , params . iterations , cipher . key_len () , hash ,) ? ; let iv = cryptography_crypto :: pkcs12 :: kdf (password , params . salt , cryptography_crypto :: pkcs12 :: KDF_IV_ID , params . iterations , cipher . block_size () , hash ,) ? ; openssl :: symm :: decrypt (cipher , & key , Some (& iv) , data) . map_err (| _ | KeyParsingError :: IncorrectPassword) }
};
}
