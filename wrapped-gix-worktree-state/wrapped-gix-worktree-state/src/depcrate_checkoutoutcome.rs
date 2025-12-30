// Generated macro for Outcome (struct)
macro_rules! Depcrate_checkoutOutcome {
() => {
// Module: crate::checkout
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The outcome of checking out an entire index."] # [derive (Debug , Default)] pub struct Outcome { # [doc = " The amount of files updated, or created."] pub files_updated : usize , # [doc = " The amount of bytes written to disk,"] pub bytes_written : u64 , # [doc = " The encountered collisions, which can happen on a case-insensitive filesystem."] pub collisions : Vec < Collision > , # [doc = " Other errors that happened during checkout."] pub errors : Vec < ErrorRecord > , # [doc = " Relative paths that the process listed as 'delayed' even though we never passed them."] pub delayed_paths_unknown : Vec < BString > , # [doc = " All paths that were left unprocessed, because they were never listed by the process even though we passed them."] pub delayed_paths_unprocessed : Vec < BString > , }
};
}
