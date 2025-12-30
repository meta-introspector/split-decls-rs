// Generated macro for other_389 (other)
macro_rules! Depcrate_crypto_boringsslother_389 {
() => {
// Module: crate::crypto::boringssl
// Provides: {"other_389"}
// Dependencies: {}
extern "C" { fn EVP_aead_aes_128_gcm_tls13 () -> * const EVP_AEAD ; fn EVP_aead_aes_256_gcm_tls13 () -> * const EVP_AEAD ; fn EVP_aead_chacha20_poly1305 () -> * const EVP_AEAD ; fn HKDF_extract (out_key : * mut u8 , out_len : * mut usize , digest : * const EVP_MD , secret : * const u8 , secret_len : usize , salt : * const u8 , salt_len : usize ,) -> c_int ; fn HKDF_expand (out_key : * mut u8 , out_len : usize , digest : * const EVP_MD , prk : * const u8 , prk_len : usize , info : * const u8 , info_len : usize ,) -> c_int ; fn EVP_AEAD_CTX_init (ctx : * mut EVP_AEAD_CTX , aead : * const EVP_AEAD , key : * const u8 , key_len : usize , tag_len : usize , engine : * mut c_void ,) -> c_int ; fn EVP_AEAD_CTX_open (ctx : * const EVP_AEAD_CTX , out : * mut u8 , out_len : * mut usize , max_out_len : usize , nonce : * const u8 , nonce_len : usize , inp : * const u8 , in_len : usize , ad : * const u8 , ad_len : usize ,) -> c_int ; fn EVP_AEAD_CTX_seal_scatter (ctx : * const EVP_AEAD_CTX , out : * mut u8 , out_tag : * mut u8 , out_tag_len : * mut usize , max_out_tag_len : usize , nonce : * const u8 , nonce_len : usize , inp : * const u8 , in_len : usize , extra_in : * const u8 , extra_in_len : usize , ad : * const u8 , ad_len : usize ,) -> c_int ; fn AES_set_encrypt_key (key : * const u8 , bits : c_uint , aeskey : * mut AES_KEY ,) -> c_int ; fn AES_ecb_encrypt (inp : * const u8 , out : * mut u8 , key : * const AES_KEY , enc : c_int ,) -> c_void ; fn CRYPTO_chacha_20 (out : * mut u8 , inp : * const u8 , in_len : usize , key : * const u8 , nonce : * const u8 , counter : u32 ,) -> c_void ; }
};
}
