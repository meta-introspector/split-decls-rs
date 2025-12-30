// Generated macro for Error (enum)
macro_rules! Depcrate_decodeError {
() => {
// Module: crate::decode
// Provides: {"Error"}
// Dependencies: {}
# [doc = " This type represents all possible errors that can occur when deserializing a value."] # [derive (Debug)] pub enum Error { # [doc = " Error while reading marker byte."] InvalidMarkerRead (io :: Error) , # [doc = " Error while reading data."] InvalidDataRead (io :: Error) , # [doc = " The depth limit [`MAX_DEPTH`] was exceeded."] DepthLimitExceeded , }
};
}
