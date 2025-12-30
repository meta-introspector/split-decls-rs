// Generated macro for Error (enum)
macro_rules! Depcrate_decodeError {
() => {
// Module: crate::decode
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error used in the [`decode`][mod@crate::decode] module"] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to decode the first four hex bytes indicating the line length: {err}")] HexDecode { err : String } , # [error ("The data received claims to be larger than the maximum allowed size: got {length_in_bytes}, exceeds {MAX_DATA_LEN}")] DataLengthLimitExceeded { length_in_bytes : usize } , # [error ("Received an invalid empty line")] DataIsEmpty , # [error ("Received an invalid line of length 3")] InvalidLineLength , # [error ("{data:?} - consumed {bytes_consumed} bytes")] Line { data : BString , bytes_consumed : usize } , # [error ("Needing {bytes_needed} additional bytes to decode the line successfully")] NotEnoughData { bytes_needed : usize } , }
};
}
