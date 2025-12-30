// Generated macro for encrypt_pem (function)
macro_rules! Depcrate_pemencrypt_pem {
() => {
// Module: crate::pem
// Provides: {"encrypt_pem"}
// Dependencies: {}
# [doc = " Encrypts DER data with legacy PEM encryption (Proc-Type and DEK-Info"] # [doc = " headers). Returns PEM-formatted bytes with encryption headers."] # [doc = ""] # [doc = " If password is empty, returns unencrypted PEM. Otherwise, encrypts using"] # [doc = " AES-256-CBC."] pub fn encrypt_pem (tag : & str , der_data : & [u8] , password : & [u8] ,) -> crate :: KeySerializationResult < Vec < u8 > > { if password . is_empty () { let pem = pem :: Pem :: new (tag , der_data) ; return Ok (pem :: encode_config (& pem , ENCODE_CONFIG) . into_bytes ()) ; } let cipher = openssl :: symm :: Cipher :: aes_256_cbc () ; let iv_len = cipher . iv_len () . unwrap () ; let mut iv = vec ! [0u8 ; iv_len] ; cryptography_openssl :: rand :: rand_bytes (& mut iv) ? ; let key = cryptography_crypto :: pbkdf1 :: openssl_kdf (openssl :: hash :: MessageDigest :: md5 () , password , iv . get (.. 8) . unwrap () . try_into () . unwrap () , cipher . key_len () ,) ? ; let encrypted = openssl :: symm :: encrypt (cipher , & key , Some (& iv) , der_data) ? ; let iv_hex = cryptography_crypto :: encoding :: hex_encode (& iv) ; let mut pem = pem :: Pem :: new (tag , encrypted) ; pem . headers_mut () . add ("Proc-Type" , "4,ENCRYPTED") . unwrap () ; pem . headers_mut () . add ("DEK-Info" , & format ! ("AES-256-CBC,{iv_hex}")) . unwrap () ; Ok (pem :: encode_config (& pem , ENCODE_CONFIG) . into_bytes ()) }
};
}
