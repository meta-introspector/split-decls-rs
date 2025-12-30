// Generated macro for Error (enum)
macro_rules! Depcrate_alternate_parseError {
() => {
// Module: crate::alternate::parse
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Returned as part of [`crate::alternate::Error::Parse`]"] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("Could not obtain an object path for the alternate directory '{}'" , String :: from_utf8_lossy (. 0))] PathConversion (Vec < u8 >) , # [error ("Could not unquote alternate path")] Unquote (# [from] gix_quote :: ansi_c :: undo :: Error) , }
};
}
