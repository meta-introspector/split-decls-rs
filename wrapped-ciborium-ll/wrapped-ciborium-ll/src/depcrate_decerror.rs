// Generated macro for Error (enum)
macro_rules! Depcrate_decError {
() => {
// Module: crate::dec
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error that occurred while decoding"] # [derive (Clone , Debug)] pub enum Error < T > { # [doc = " An error occurred while reading bytes"] # [doc = ""] # [doc = " Contains the underlying error returned while reading."] Io (T) , # [doc = " An error occurred while parsing bytes"] # [doc = ""] # [doc = " Contains the offset into the stream where the syntax error occurred."] Syntax (usize) , }
};
}
