macro_rules! deps {
    () => {
        Poly1305!();
    };
}

macro_rules! macro_257 {
    () => {
        deps!();
        construct_secret_key ! { # [doc = " A type to represent the `OneTimeKey` that Poly1305 uses for authentication."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] # [doc = ""] # [doc = " # Panics:"] # [doc = " A panic will occur if:"] # [doc = " - Failure to generate random bytes securely."] (OneTimeKey , test_one_time_key , POLY1305_KEYSIZE , POLY1305_KEYSIZE , POLY1305_KEYSIZE) }
    };
}

macro_257!();