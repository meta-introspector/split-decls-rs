// Generated macro for tests (module)
macro_rules! Depcrate_aestests {
() => {
// Module: crate::aes
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { aes :: { DecryptKey , EncryptKey } , test_helpers :: decode_hex , } ; # [test] fn aes_128 () { let key = decode_hex ("2b7e151628aed2a6abf7158809cf4f3c") ; let plaintext = decode_hex ("6bc1bee22e409f96e93d7e117393172a") ; let ciphertext = decode_hex ("3ad77bb40d7a3660a89ecaf32466ef97") ; assert_eq ! (ciphertext , EncryptKey :: new_128 (& key) . encrypt (& plaintext)) ; assert_eq ! (plaintext , DecryptKey :: new_128 (& key) . decrypt (& ciphertext)) ; } # [test] fn aes_256 () { let key = decode_hex ("603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4") ; let plaintext = decode_hex ("6bc1bee22e409f96e93d7e117393172a") ; let ciphertext = decode_hex ("f3eed1bdb5d2a03c064b5a7e3db181f8") ; assert_eq ! (ciphertext , EncryptKey :: new_256 (& key) . encrypt (& plaintext)) ; assert_eq ! (plaintext , DecryptKey :: new_256 (& key) . decrypt (& ciphertext)) ; } }
};
}
