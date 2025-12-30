// Generated macro for encrypt_decrypt_out_length (function)
macro_rules! Depcrate_test_framework_streamcipher_interfaceencrypt_decrypt_out_length {
() => {
// Module: crate::test_framework::streamcipher_interface
// Provides: {"encrypt_decrypt_out_length"}
// Dependencies: {}
# [cfg (feature = "safe_api")] fn encrypt_decrypt_out_length < Encryptor , Decryptor , Key , Nonce > (encryptor : & Encryptor , decryptor : & Decryptor , key : & Key , nonce : & Nonce , input : & [u8] ,) where Encryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , Decryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , { assert ! (! input . is_empty ()) ; let mut dst_out_empty = vec ! [0u8 ; 0] ; assert ! (encryptor (key , nonce , 0 , input , & mut dst_out_empty) . is_err ()) ; assert ! (decryptor (key , nonce , 0 , input , & mut dst_out_empty) . is_err ()) ; let mut dst_out_less = vec ! [0u8 ; input . len () - 1] ; assert ! (encryptor (key , nonce , 0 , input , & mut dst_out_less) . is_err ()) ; assert ! (decryptor (key , nonce , 0 , input , & mut dst_out_less) . is_err ()) ; let mut dst_out_exact = vec ! [0u8 ; input . len ()] ; assert ! (encryptor (key , nonce , 0 , input , & mut dst_out_exact) . is_ok ()) ; assert ! (decryptor (key , nonce , 0 , input , & mut dst_out_exact) . is_ok ()) ; let mut dst_out_greater = vec ! [0u8 ; input . len () + 1] ; assert ! (encryptor (key , nonce , 0 , input , & mut dst_out_greater) . is_ok ()) ; assert ! (decryptor (key , nonce , 0 , input , & mut dst_out_greater) . is_ok ()) ; }
};
}
