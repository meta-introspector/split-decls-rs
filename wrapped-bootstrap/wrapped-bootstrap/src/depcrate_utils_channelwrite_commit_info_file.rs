// Generated macro for write_commit_info_file (function)
macro_rules! Depcrate_utils_channelwrite_commit_info_file {
() => {
// Module: crate::utils::channel
// Provides: {"write_commit_info_file"}
// Dependencies: {}
# [doc = " Write the commit information to the `git-commit-info` file given the project"] # [doc = " root."] pub fn write_commit_info_file (root : & Path , info : & Info) { let commit_info = format ! ("{}\n{}\n{}\n" , info . sha , info . short_sha , info . commit_date) ; t ! (fs :: write (root . join ("git-commit-info") , commit_info)) ; }
};
}
