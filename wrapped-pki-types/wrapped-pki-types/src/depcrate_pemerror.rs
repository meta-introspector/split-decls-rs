// Generated macro for Error (enum)
macro_rules! Depcrate_pemError {
() => {
// Module: crate::pem
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Errors that may arise when parsing the contents of a PEM file"] # [non_exhaustive] # [derive (Debug)] pub enum Error { # [doc = " a section is missing its \"END marker\" line"] MissingSectionEnd { # [doc = " the expected \"END marker\" line that was not found"] end_marker : Vec < u8 > , } , # [doc = " syntax error found in the line that starts a new section"] IllegalSectionStart { # [doc = " line that contains the syntax error"] line : Vec < u8 > , } , # [doc = " base64 decode error"] Base64Decode (String) , # [doc = " I/O errors, from APIs that accept `std::io` types."] # [cfg (feature = "std")] Io (io :: Error) , # [doc = " No items found of desired type"] NoItemsFound , }
};
}
