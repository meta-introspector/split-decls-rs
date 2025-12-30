// Generated macro for return_if_counter_will_overflow (function)
macro_rules! Depcrate_test_framework_streamcipher_interfacereturn_if_counter_will_overflow {
() => {
// Module: crate::test_framework::streamcipher_interface
// Provides: {"return_if_counter_will_overflow"}
// Dependencies: {}
# [cfg (feature = "safe_api")] fn return_if_counter_will_overflow < Encryptor , Decryptor , Key , Nonce > (encryptor : & Encryptor , decryptor : & Decryptor , key : & Key , nonce : & Nonce , counter : u32 , input : & [u8] ,) -> bool where Encryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , Decryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , { assert ! (! input . is_empty ()) ; let mut dst_out = vec ! [0u8 ; input . len ()] ; let enc_res = encryptor (key , nonce , counter , & [0u8 ; 0] , & mut dst_out) . is_err () ; let dec_res = decryptor (key , nonce , counter , & [0u8 ; 0] , & mut dst_out) . is_err () ; enc_res && dec_res }
};
}
