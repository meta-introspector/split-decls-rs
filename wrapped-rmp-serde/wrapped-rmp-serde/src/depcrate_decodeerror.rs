// Generated macro for Error (enum)
macro_rules! Depcrate_decodeError {
() => {
// Module: crate::decode
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Enum representing errors that can occur while decoding MessagePack data."] # [derive (Debug)] pub enum Error { # [doc = " The enclosed I/O error occurred while trying to read a MessagePack"] # [doc = " marker."] InvalidMarkerRead (io :: Error) , # [doc = " The enclosed I/O error occurred while trying to read the encoded"] # [doc = " MessagePack data."] InvalidDataRead (io :: Error) , # [doc = " A mismatch occurred between the decoded and expected value types."] TypeMismatch (Marker) , # [doc = " A numeric cast failed due to an out-of-range error."] OutOfRange , # [doc = " A decoded array did not have the enclosed expected length."] LengthMismatch (u32) , # [doc = " An otherwise uncategorized error occurred. See the enclosed `String` for"] # [doc = " details."] Uncategorized (String) , # [doc = " A general error occurred while deserializing the expected type. See the"] # [doc = " enclosed `String` for details."] Syntax (String) , # [doc = " An encoded string could not be parsed as UTF-8."] Utf8Error (Utf8Error) , # [doc = " The depth limit was exceeded."] DepthLimitExceeded , }
};
}
