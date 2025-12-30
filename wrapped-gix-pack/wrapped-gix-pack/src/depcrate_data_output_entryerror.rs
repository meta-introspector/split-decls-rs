// Generated macro for Error (enum)
macro_rules! Depcrate_data_output_entryError {
() => {
// Module: crate::data::output::entry
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`output::Entry::from_data()`]."] # [allow (missing_docs)] # [derive (Debug , thiserror :: Error)] pub enum Error { # [error ("{0}")] ZlibDeflate (# [from] std :: io :: Error) , # [error (transparent)] EntryType (# [from] crate :: data :: entry :: decode :: Error) , }
};
}
