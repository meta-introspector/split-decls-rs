// Generated macro for test (module)
macro_rules! Depcrate_ciphertest {
() => {
// Module: crate::cipher
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: cipher :: { CipherInitPurpose , EvpAes128Cbc , EvpAes128Ctr } ; use super :: Cipher ; # [test] fn test_cipher_mode () { assert_eq ! (Cipher ::< EvpAes128Ctr >:: new (& [0 ; 16] , & [0 ; 16] , CipherInitPurpose :: Encrypt) . cipher_mode () , bssl_sys :: EVP_CIPH_CTR_MODE as u32) ; assert_eq ! (Cipher ::< EvpAes128Cbc >:: new (& [0 ; 16] , & [0 ; 16] , CipherInitPurpose :: Encrypt) . cipher_mode () , bssl_sys :: EVP_CIPH_CBC_MODE as u32) ; } # [should_panic] # [test] fn test_apply_keystream_on_cbc () { let mut cipher = Cipher :: < EvpAes128Cbc > :: new (& [0 ; 16] , & [0 ; 16] , CipherInitPurpose :: Encrypt) ; let mut buf = [0 ; 16] ; let _ = cipher . apply_keystream_in_place (& mut buf) ; } }
};
}
