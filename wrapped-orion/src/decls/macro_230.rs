macro_rules! macro_230 {
    () => {
        construct_public ! { # [doc = " A type to represent the `Digest` that BLAKE2b returns."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is empty."] # [doc = " - `slice` is greater than 64 bytes."] (Digest , test_digest , 1 , BLAKE2B_OUTSIZE) }
    };
}

macro_230!();