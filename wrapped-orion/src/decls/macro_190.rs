macro_rules! macro_190 {
    () => {
        construct_public ! { # [doc = " A type to represent the `Digest` that SHA3-512 returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 64 bytes."] (Digest , test_digest , SHA3_512_OUTSIZE , SHA3_512_OUTSIZE) }
    };
}

macro_190!()