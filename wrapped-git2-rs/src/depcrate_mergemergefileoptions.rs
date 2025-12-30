// Generated macro for MergeFileOptions (struct)
macro_rules! Depcrate_mergeMergeFileOptions {
() => {
// Module: crate::merge
// Provides: {"MergeFileOptions"}
// Dependencies: {}
# [doc = " Options for merging a file."] pub struct MergeFileOptions { ancestor_label : Option < CString > , our_label : Option < CString > , their_label : Option < CString > , raw : raw :: git_merge_file_options , }
};
}
