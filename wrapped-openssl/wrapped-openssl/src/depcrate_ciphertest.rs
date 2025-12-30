// Generated macro for test (module)
macro_rules! Depcrate_ciphertest {
() => {
// Module: crate::cipher
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [cfg (ossl300)] use super :: Cipher ; # [test] # [cfg (ossl300)] fn test_cipher_fetch_properties () { assert ! (Cipher :: fetch (None , "AES-128-GCM" , Some ("provider=gibberish")) . is_err ()) ; } }
};
}
