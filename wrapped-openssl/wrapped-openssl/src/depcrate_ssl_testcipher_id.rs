// Generated macro for cipher_id (function)
macro_rules! Depcrate_ssl_testcipher_id {
() => {
// Module: crate::ssl::test
// Provides: {"cipher_id"}
// Dependencies: {}
# [test] # [cfg (ossl111)] fn cipher_id () { let mut server = Server :: builder () ; server . ctx () . set_ciphersuites ("TLS_AES_256_GCM_SHA384") . unwrap () ; let server = server . build () ; let client = server . client () ; let s = client . connect () ; let ssl = s . ssl () ; let cipher = ssl . current_cipher () . unwrap () ; let cipher_id = cipher . protocol_id () ; assert_eq ! (cipher_id , [0x13 , 0x02]) ; }
};
}
