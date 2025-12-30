// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type used by `blobby` functions"] # [derive (Debug , Eq , PartialEq , Copy , Clone)] pub enum Error { # [doc = " Decoded VLQ number is too big"] InvalidVlq , # [doc = " Invalid de-duplicated blob index"] InvalidIndex , # [doc = " Unexpected end of data"] UnexpectedEnd , # [doc = " Not enough elements for `BlobNIterator`"] NotEnoughElements , # [doc = " Bad array length was provided to [`parse_as_array`]"] BadArrayLen , }
};
}
