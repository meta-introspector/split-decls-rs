// Generated macro for Error (enum)
macro_rules! Depcrate_worktree_encode_to_gitError {
() => {
// Module: crate::worktree::encode_to_git
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`encode_to_git()][super::encode_to_git()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Cannot convert input of {input_len} bytes to UTF-8 without overflowing")] Overflow { input_len : usize } , # [error ("The input was malformed and could not be decoded as '{encoding}'")] Malformed { encoding : & 'static str } , # [error ("Encoding from '{src_encoding}' to '{dest_encoding}' and back is not the same")] RoundTrip { src_encoding : & 'static str , dest_encoding : & 'static str , } , }
};
}
