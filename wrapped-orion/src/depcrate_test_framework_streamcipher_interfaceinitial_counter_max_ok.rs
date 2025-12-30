// Generated macro for initial_counter_max_ok (function)
macro_rules! Depcrate_test_framework_streamcipher_interfaceinitial_counter_max_ok {
() => {
// Module: crate::test_framework::streamcipher_interface
// Provides: {"initial_counter_max_ok"}
// Dependencies: {}
# [cfg (feature = "safe_api")] # [doc = " Test that processing one block does not fail on the largest possible initial block counter."] fn initial_counter_max_ok < Encryptor , Decryptor , Key , Nonce > (encryptor : & Encryptor , decryptor : & Decryptor , key : & Key , nonce : & Nonce ,) where Encryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , Decryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , { let mut dst_out = [0u8 ; 64] ; assert ! (encryptor (key , nonce , u32 :: MAX , & [0u8 ; 64] , & mut dst_out) . is_ok ()) ; assert ! (decryptor (key , nonce , u32 :: MAX , & [0u8 ; 64] , & mut dst_out) . is_ok ()) ; }
};
}
