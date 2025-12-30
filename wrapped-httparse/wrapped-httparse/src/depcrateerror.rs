// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error in parsing."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum Error { # [doc = " Invalid byte in header name."] HeaderName , # [doc = " Invalid byte in header value."] HeaderValue , # [doc = " Invalid byte in new line."] NewLine , # [doc = " Invalid byte in Response status."] Status , # [doc = " Invalid byte where token is required."] Token , # [doc = " Parsed more headers than provided buffer can contain."] TooManyHeaders , # [doc = " Invalid byte in HTTP version."] Version , }
};
}
