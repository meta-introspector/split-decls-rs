// Generated macro for write_commit_hash_file (function)
macro_rules! Depcrate_utils_channelwrite_commit_hash_file {
() => {
// Module: crate::utils::channel
// Provides: {"write_commit_hash_file"}
// Dependencies: {}
# [doc = " Write the commit hash to the `git-commit-hash` file given the project root."] pub fn write_commit_hash_file (root : & Path , sha : & str) { t ! (fs :: write (root . join ("git-commit-hash") , sha)) ; }
};
}
