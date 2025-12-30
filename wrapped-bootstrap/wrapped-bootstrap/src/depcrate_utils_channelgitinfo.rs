// Generated macro for GitInfo (enum)
macro_rules! Depcrate_utils_channelGitInfo {
() => {
// Module: crate::utils::channel
// Provides: {"GitInfo"}
// Dependencies: {}
# [derive (Clone , Default)] pub enum GitInfo { # [doc = " This is not a git repository."] # [default] Absent , # [doc = " This is a git repository."] # [doc = " If the info should be used (`omit_git_hash` is false), this will be"] # [doc = " `Some`, otherwise it will be `None`."] Present (Option < Info >) , # [doc = " This is not a git repository, but the info can be fetched from the"] # [doc = " `git-commit-info` file."] RecordedForTarball (Info) , }
};
}
