// Generated macro for macro_422 (macro)
macro_rules! Depcrate_hazardous_stream_chacha20macro_422 {
() => {
// Module: crate::hazardous::stream::chacha20
// Provides: {"macro_422"}
// Dependencies: {}
construct_secret_key ! { # [doc = " A type to represent the `SecretKey` that Chacha20, XChaCha20, ChaCha20-Poly1305 and"] # [doc = " XChaCha20-Poly1305 use."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] # [doc = ""] # [doc = " # Panics:"] # [doc = " A panic will occur if:"] # [doc = " - Failure to generate random bytes securely."] (SecretKey , test_secret_key , CHACHA_KEYSIZE , CHACHA_KEYSIZE , CHACHA_KEYSIZE) }
};
}
