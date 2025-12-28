macro_rules! macro_170 {
    () => {
        construct_public ! { # [doc = " A type to represent the `Digest` that SHA3-256 returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] (Digest , test_digest , SHA3_256_OUTSIZE , SHA3_256_OUTSIZE) }
    };
}

macro_170!()