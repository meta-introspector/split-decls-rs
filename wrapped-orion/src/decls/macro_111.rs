macro_rules! macro_111 {
    () => {
        construct_public ! { # [doc = " A type to represent the `Digest` that SHA256 returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] (Digest , test_digest , SHA256_OUTSIZE , SHA256_OUTSIZE) }
    };
}

macro_111!();