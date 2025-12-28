macro_rules! macro_616 {
    () => {
        construct_secret_key_variable_size ! { # [doc = " A type to represent a secret key."] # [doc = ""] # [doc = " As default it will randomly generate a `SecretKey` of 32 bytes."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is empty."] # [doc = " - `length` is 0."] # [doc = " - `length` is not less than [`isize::MAX`]."] # [doc = ""] # [doc = " # Panics:"] # [doc = " A panic will occur if:"] # [doc = " - Failure to generate random bytes securely."] (SecretKey , test_secret_key , 32) }
    };
}

macro_616!()