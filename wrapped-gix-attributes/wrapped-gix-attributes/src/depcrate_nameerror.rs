// Generated macro for Error (struct)
macro_rules! Depcrate_nameError {
() => {
// Module: crate::name
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`parse::Iter`][crate::parse::Iter]."] # [derive (Debug , thiserror :: Error)] # [error ("Attribute has non-ascii characters or starts with '-': {attribute}")] pub struct Error { # [doc = " The attribute that failed to parse."] pub attribute : BString , }
};
}
