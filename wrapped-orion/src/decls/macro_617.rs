macro_rules! macro_617 {
    () => {
        construct_salt_variable_size ! { # [doc = " A type to represent the `Salt` that Argon2i uses during key derivation."] # [doc = ""] # [doc = " As default it will randomly generate a `Salt` of 16 bytes."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is empty."] # [doc = " - `length` is 0."] # [doc = " - `length` is not less than [`isize::MAX`]."] # [doc = ""] # [doc = " # Panics:"] # [doc = " A panic will occur if:"] # [doc = " - Failure to generate random bytes securely."] (Salt , test_salt , 16) }
    };
}

macro_617!()