macro_rules! macro_549 {
    () => {
        construct_secret_key ! { # [doc = " A type to represent the private `SharedSecret` that X-Wing returns."] # [doc = ""] # [doc = " This type simply holds bytes. Creating an instance from slices or similar,"] # [doc = " performs no checks whatsoever."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] (SharedSecret , test_sharedsecret_key , SHARED_SECRET_SIZE , SHARED_SECRET_SIZE) }
    };
}

macro_549!()