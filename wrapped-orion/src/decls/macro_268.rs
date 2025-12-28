macro_rules! macro_268 {
    () => {
        construct_secret_key ! { # [doc = " A type to represent the secret key that BLAKE2b uses for keyed mode."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is empty."] # [doc = " - `slice` is greater than 64 bytes."] # [doc = ""] # [doc = " # Panics:"] # [doc = " A panic will occur if:"] # [doc = " - Failure to generate random bytes securely."] (SecretKey , test_secret_key , 1 , BLAKE2B_KEYSIZE , 32) }
    };
}

macro_268!()