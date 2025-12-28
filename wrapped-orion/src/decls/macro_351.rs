macro_rules! macro_351 {
    () => {
        construct_public ! { # [doc = " A type that represents a `Nonce` that XChaCha20, XChaCha20-Poly1305 use."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 24 bytes."] # [doc = ""] # [doc = " # Panics:"] # [doc = " A panic will occur if:"] # [doc = " - Failure to generate random bytes securely."] (Nonce , test_nonce , XCHACHA_NONCESIZE , XCHACHA_NONCESIZE , XCHACHA_NONCESIZE) }
    };
}

macro_351!()