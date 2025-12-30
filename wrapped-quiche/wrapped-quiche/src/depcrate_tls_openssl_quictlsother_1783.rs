// Generated macro for other_1783 (other)
macro_rules! Depcrate_tls_openssl_quictlsother_1783 {
() => {
// Module: crate::tls::openssl_quictls
// Provides: {"other_1783"}
// Dependencies: {}
extern "C" { fn SSL_CTX_ctrl (ctx : * mut SSL_CTX , cmd : c_int , larg : c_long , parg : * mut c_void ,) -> c_long ; fn SSL_get_peer_cert_chain (ssl : * const SSL) -> * mut STACK_OF ; fn SSL_get0_peer_certificate (ssl : * const SSL) -> * mut X509 ; fn SSL_ctrl (ssl : * const SSL , cmd : c_int , larg : c_long , parg : * mut c_void ,) -> c_long ; fn i2d_X509 (px : * const X509 , out : * mut * mut c_uchar) -> c_int ; fn OPENSSL_sk_num (stack : * const OPENSSL_STACK) -> usize ; fn OPENSSL_sk_value (stack : * const OPENSSL_STACK , idx : usize) -> * mut c_void ; fn CRYPTO_get_ex_new_index (class_index : c_int , argl : c_long , argp : * const c_void , new_func : * const c_void , dup_func : * const c_void , free_func : * const c_void ,) -> c_int ; fn d2i_SSL_SESSION (a : * mut * mut SSL_SESSION , pp : * mut * const c_uchar , len : c_long ,) -> * mut SSL_SESSION ; pub (super) fn i2d_SSL_SESSION (in_ : * mut SSL_SESSION , pp : * mut * mut c_uchar ,) -> c_int ; fn SSL_group_to_name (ssl : * const SSL , id : c_int) -> * const c_char ; }
};
}
