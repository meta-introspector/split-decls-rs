// Generated macro for macro_1256 (macro)
macro_rules! Depcrate_stackmacro_1256 {
() => {
// Module: crate::stack
// Provides: {"macro_1256"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , boringssl , awslc))] { use ffi :: { OPENSSL_sk_pop , OPENSSL_sk_free , OPENSSL_sk_num , OPENSSL_sk_value , OPENSSL_STACK , OPENSSL_sk_new_null , OPENSSL_sk_push , } ; } else { use ffi :: { sk_pop as OPENSSL_sk_pop , sk_free as OPENSSL_sk_free , sk_num as OPENSSL_sk_num , sk_value as OPENSSL_sk_value , _STACK as OPENSSL_STACK , sk_new_null as OPENSSL_sk_new_null , sk_push as OPENSSL_sk_push , } ; } }
};
}
