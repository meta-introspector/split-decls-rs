macro_rules! macro_180 {
    () => {
        construct_public ! { # [doc = " A type to represent the `Digest` that SHA3-384 returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 48 bytes."] (Digest , test_digest , SHA3_384_OUTSIZE , SHA3_384_OUTSIZE) }
    };
}

macro_180!();