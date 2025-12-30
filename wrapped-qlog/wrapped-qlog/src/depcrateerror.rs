// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " A quiche qlog error."] # [derive (Debug)] pub enum Error { # [doc = " There is no more work to do."] Done , # [doc = " The operation cannot be completed because it was attempted"] # [doc = " in an invalid state."] InvalidState , InvalidFormat , # [doc = " I/O error."] IoError (std :: io :: Error) , }
};
}
