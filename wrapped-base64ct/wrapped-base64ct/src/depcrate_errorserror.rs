// Generated macro for Error (enum)
macro_rules! Depcrate_errorsError {
() => {
// Module: crate::errors
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Generic error, union of [`InvalidLengthError`] and [`InvalidEncodingError`]."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum Error { # [doc = " Invalid encoding of provided Base64 string."] InvalidEncoding , # [doc = " Insufficient output buffer length."] InvalidLength , }
};
}
