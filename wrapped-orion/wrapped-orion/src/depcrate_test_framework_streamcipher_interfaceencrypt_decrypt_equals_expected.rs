// Generated macro for encrypt_decrypt_equals_expected (function)
macro_rules! Depcrate_test_framework_streamcipher_interfaceencrypt_decrypt_equals_expected {
() => {
// Module: crate::test_framework::streamcipher_interface
// Provides: {"encrypt_decrypt_equals_expected"}
// Dependencies: {}
# [cfg (feature = "safe_api")] # [doc = " Test that encrypting and decrypting produces expected plaintext/ciphertext."] fn encrypt_decrypt_equals_expected < Encryptor , Decryptor , Key , Nonce > (encryptor : & Encryptor , decryptor : & Decryptor , key : & Key , nonce : & Nonce , counter : u32 , input : & [u8] , expected_ct : Option < & [u8] > ,) where Encryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , Decryptor : Fn (& Key , & Nonce , u32 , & [u8] , & mut [u8]) -> Result < () , UnknownCryptoError > , { assert ! (! input . is_empty ()) ; if counter_increase_times (input . len () as f32) . checked_add (counter) . is_none () { assert ! (return_if_counter_will_overflow (encryptor , decryptor , key , nonce , counter , input)) ; return ; } let mut dst_out_ct = vec ! [0u8 ; input . len ()] ; encryptor (key , nonce , counter , input , & mut dst_out_ct) . unwrap () ; if let Some (expected_result) = expected_ct { assert_eq ! (expected_result , & dst_out_ct [..]) ; } let mut dst_out_pt = vec ! [0u8 ; input . len ()] ; decryptor (key , nonce , counter , & dst_out_ct , & mut dst_out_pt) . unwrap () ; assert_eq ! (input , & dst_out_pt [..]) ; if let Some (expected_result) = expected_ct { decryptor (key , nonce , counter , expected_result , & mut dst_out_pt) . unwrap () ; assert_eq ! (input , & dst_out_pt [..]) ; } }
};
}
