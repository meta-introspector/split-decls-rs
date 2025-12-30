// Generated macro for Error (struct)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Represents an internal error that occurred, with an explanation."] # [derive (Clone , Debug)] pub struct Error { # [doc = " Describes the kind of error that occurred."] kind : ErrorKind , # [doc = " More explanation of error that occurred."] message : Cow < 'static , str > , }
};
}
