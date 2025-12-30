// Generated macro for Error (struct)
macro_rules! Depcrate_data_entry_decodeError {
() => {
// Module: crate::data::entry::decode
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [data::Entry::from_bytes()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] # [error ("Object type {type_id} is unsupported")] pub struct Error { pub type_id : u8 , }
};
}
