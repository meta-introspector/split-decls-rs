// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " This type represents all possible errors that can occur when serializing or"] # [doc = " deserializing JSON data."] pub struct Error { # [doc = " This `Box` allows us to keep the size of `Error` as small as possible. A"] # [doc = " larger `Error` type was substantially slower due to all the functions"] # [doc = " that pass around `Result<T, Error>`."] err : Box < ErrorImpl > , }
};
}
