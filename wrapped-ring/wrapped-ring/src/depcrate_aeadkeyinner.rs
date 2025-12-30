// Generated macro for KeyInner (enum)
macro_rules! Depcrate_aeadKeyInner {
() => {
// Module: crate::aead
// Provides: {"KeyInner"}
// Dependencies: {}
# [allow (clippy :: large_enum_variant , variant_size_differences)] # [derive (Clone)] enum KeyInner { AesGcm (aes_gcm :: Key) , ChaCha20Poly1305 (chacha20_poly1305 :: Key) , }
};
}
