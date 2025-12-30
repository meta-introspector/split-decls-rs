// Generated macro for openssl_cipher_name (function)
macro_rules! Depcrate_ssl_testopenssl_cipher_name {
() => {
// Module: crate::ssl::test
// Provides: {"openssl_cipher_name"}
// Dependencies: {}
# [test] # [cfg (ossl111)] fn openssl_cipher_name () { assert_eq ! (super :: cipher_name ("TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384") , "ECDHE-RSA-AES256-SHA384" ,) ; assert_eq ! (super :: cipher_name ("asdf") , "(NONE)") ; }
};
}
