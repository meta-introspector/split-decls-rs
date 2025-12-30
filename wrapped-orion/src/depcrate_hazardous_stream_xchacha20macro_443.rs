// Generated macro for macro_443 (macro)
macro_rules! Depcrate_hazardous_stream_xchacha20macro_443 {
() => {
// Module: crate::hazardous::stream::xchacha20
// Provides: {"macro_443"}
// Dependencies: {}
construct_public ! { # [doc = " A type that represents a `Nonce` that XChaCha20, XChaCha20-Poly1305 use."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 24 bytes."] # [doc = ""] # [doc = " # Panics:"] # [doc = " A panic will occur if:"] # [doc = " - Failure to generate random bytes securely."] (Nonce , test_nonce , XCHACHA_NONCESIZE , XCHACHA_NONCESIZE , XCHACHA_NONCESIZE) }
};
}
