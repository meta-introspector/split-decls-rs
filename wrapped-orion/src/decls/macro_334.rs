macro_rules! deps {
    () => {
        ChaCha20!();
    };
}

macro_rules! macro_334 {
    () => {
        deps!();
        construct_public ! { # [doc = " A type that represents a `Nonce` that ChaCha20 and ChaCha20-Poly1305 use."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 12 bytes."] (Nonce , test_nonce , IETF_CHACHA_NONCESIZE , IETF_CHACHA_NONCESIZE) }
    };
}

macro_334!()