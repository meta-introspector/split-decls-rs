// Generated macro for encrypt_decrypt_input_empty (function)
macro_rules! Depcrate_test_framework_streamcipher_interfaceencrypt_decrypt_input_empty {
() => {
// Module: crate::test_framework::streamcipher_interface
// Provides: {"encrypt_decrypt_input_empty"}
// Dependencies: {}
# [cfg (feature = "safe_api")] fn encrypt_decrypt_input_empty < Encryptor , Decryptor , Key , Nonce > (encryptor : & Encryptor , decryptor : & Decryptor , key : & Key , nonce : & Nonce ,) where Encryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , Decryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , { let mut dst_out = [0u8 ; 64] ; assert ! (encryptor (key , nonce , 0 , & [0u8 ; 0] , & mut dst_out) . is_err ()) ; assert ! (decryptor (key , nonce , 0 , & [0u8 ; 0] , & mut dst_out) . is_err ()) ; }
};
}
