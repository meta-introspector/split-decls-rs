macro_rules! macro_123 {
    () => {
        construct_public ! { # [doc = " A type to represent the `Digest` that SHA384 returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 48 bytes."] (Digest , test_digest , SHA384_OUTSIZE , SHA384_OUTSIZE) }
    };
}

macro_123!();