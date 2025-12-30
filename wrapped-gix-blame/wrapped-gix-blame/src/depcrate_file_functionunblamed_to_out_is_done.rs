// Generated macro for unblamed_to_out_is_done (function)
macro_rules! Depcrate_file_functionunblamed_to_out_is_done {
() => {
// Module: crate::file::function
// Provides: {"unblamed_to_out_is_done"}
// Dependencies: {}
# [doc = " Convert each of the unblamed hunk in `hunks_to_blame` into a [`BlameEntry`], consuming them in the process."] # [doc = ""] # [doc = " Return `true` if we are done because `hunks_to_blame` is empty."] fn unblamed_to_out_is_done (hunks_to_blame : & mut Vec < UnblamedHunk > , out : & mut Vec < BlameEntry > , suspect : ObjectId ,) -> bool { let mut without_suspect = Vec :: new () ; out . extend (hunks_to_blame . drain (..) . filter_map (| hunk | { BlameEntry :: from_unblamed_hunk (& hunk , suspect) . or_else (| | { without_suspect . push (hunk) ; None }) })) ; * hunks_to_blame = without_suspect ; hunks_to_blame . is_empty () }
};
}
