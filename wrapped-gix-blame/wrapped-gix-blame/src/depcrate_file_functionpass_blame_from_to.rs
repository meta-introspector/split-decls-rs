// Generated macro for pass_blame_from_to (function)
macro_rules! Depcrate_file_functionpass_blame_from_to {
() => {
// Module: crate::file::function
// Provides: {"pass_blame_from_to"}
// Dependencies: {}
# [doc = " Pass ownership of each unblamed hunk of `from` to `to`."] # [doc = ""] # [doc = " This happens when `from` didn't actually change anything in the blamed file."] fn pass_blame_from_to (from : ObjectId , to : ObjectId , hunks_to_blame : & mut Vec < UnblamedHunk >) { for unblamed_hunk in hunks_to_blame { unblamed_hunk . pass_blame (from , to) ; } }
};
}
