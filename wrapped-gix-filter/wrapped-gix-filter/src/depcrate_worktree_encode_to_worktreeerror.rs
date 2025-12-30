// Generated macro for Error (enum)
macro_rules! Depcrate_worktree_encode_to_worktreeError {
() => {
// Module: crate::worktree::encode_to_worktree
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`encode_to_worktree()][super::encode_to_worktree()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Cannot convert input of {input_len} UTF-8 bytes to target encoding without overflowing")] Overflow { input_len : usize } , # [error ("Input was not UTF-8 encoded")] InputAsUtf8 (# [from] std :: str :: Utf8Error) , # [error ("The character '{character}' could not be mapped to the {worktree_encoding}")] Unmappable { character : char , worktree_encoding : & 'static str , } , }
};
}
