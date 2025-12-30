// Generated macro for entries (module)
macro_rules! Depcrate_verifyentries {
() => {
// Module: crate::verify
// Provides: {"entries"}
// Dependencies: {}
# [doc = ""] pub mod entries { use bstr :: BString ; # [doc = " The error returned by [`State::verify_entries()`][crate::State::verify_entries()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Entry '{current_path}' (stage = {current_stage}) at index {current_index} should order after prior entry '{previous_path}' (stage = {previous_stage})")] OutOfOrder { current_index : usize , current_path : BString , current_stage : u8 , previous_path : BString , previous_stage : u8 , } , } }
};
}
