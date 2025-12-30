// Generated macro for Error (enum)
macro_rules! Depcrate_parse_section_headerError {
() => {
// Module: crate::parse::section::header
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`Header::new(…)`][super::Header::new()]."] # [derive (Debug , PartialOrd , PartialEq , Eq , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("section names can only be ascii, '-'")] InvalidName , # [error ("sub-section names must not contain newlines or null bytes")] InvalidSubSection , }
};
}
