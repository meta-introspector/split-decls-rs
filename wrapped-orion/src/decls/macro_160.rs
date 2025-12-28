macro_rules! macro_160 {
    () => {
        construct_public ! { # [doc = " A type to represent the `Digest` that SHA3-224 returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 28 bytes."] (Digest , test_digest , SHA3_224_OUTSIZE , SHA3_224_OUTSIZE) }
    };
}

macro_160!()