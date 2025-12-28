macro_rules! macro_407 {
    () => {
        construct_secret_key ! { # [doc = " A type to represent the `SharedSecret` that DH-KEM(X25519, HKDF-SHA256) produces."] # [doc = ""] # [doc = " This type simply holds bytes. Creating an instance from slices or similar,"] # [doc = " performs no checks whatsoever."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is not 32 bytes."] (SharedSecret , test_shared_key , 32 , 32) }
    };
}

macro_407!();