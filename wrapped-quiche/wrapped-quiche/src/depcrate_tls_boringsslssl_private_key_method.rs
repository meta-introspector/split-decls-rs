// Generated macro for SSL_PRIVATE_KEY_METHOD (struct)
macro_rules! Depcrate_tls_boringsslSSL_PRIVATE_KEY_METHOD {
() => {
// Module: crate::tls::boringssl
// Provides: {"SSL_PRIVATE_KEY_METHOD"}
// Dependencies: {}
# [cfg (test)] # [repr (C)] # [allow (non_camel_case_types)] struct SSL_PRIVATE_KEY_METHOD { sign : Option < unsafe extern "C" fn (ssl : * mut SSL , out : * mut u8 , out_len : * mut usize , max_out : usize , signature_algorithm : u16 , r#in : * const u8 , in_len : usize ,) -> ssl_private_key_result_t , > , decrypt : Option < unsafe extern "C" fn (ssl : * mut SSL , out : * mut u8 , out_len : * mut usize , max_out : usize , r#in : * const u8 , in_len : usize ,) -> ssl_private_key_result_t , > , complete : Option < unsafe extern "C" fn (ssl : * mut SSL , out : * mut u8 , out_len : * mut usize , max_out : usize ,) -> ssl_private_key_result_t , > , }
};
}
