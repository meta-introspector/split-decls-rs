// Generated macro for macro_819 (macro)
macro_rules! Depcrate_high_level_hltypesmacro_819 {
() => {
// Module: crate::high_level::hltypes
// Provides: {"macro_819"}
// Dependencies: {}
construct_secret_key_variable_size ! { # [doc = " A type to represent the `Password` that Argon2i hashes and uses for key derivation."] # [doc = ""] # [doc = " As default it will randomly generate a `Password` of 32 bytes."] # [doc = ""] # [doc = " # Errors:"] # [doc = " An error will be returned if:"] # [doc = " - `slice` is empty."] # [doc = " - `length` is 0."] # [doc = " - `length` is not less than [`isize::MAX`]."] # [doc = ""] # [doc = " # Panics:"] # [doc = " A panic will occur if:"] # [doc = " - Failure to generate random bytes securely."] (Password , test_password , 32) }
};
}
