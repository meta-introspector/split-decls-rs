// Generated macro for VersionInfo (struct)
macro_rules! Depcrate_versionVersionInfo {
() => {
// Module: crate::version
// Provides: {"VersionInfo"}
// Dependencies: {}
# [doc = " Cargo's version."] pub struct VersionInfo { # [doc = " rust-analyzer's version, such as \"1.57.0\", \"1.58.0-beta.1\", \"1.59.0-nightly\", etc."] pub version : & 'static str , # [doc = " The release channel we were built for (stable/beta/nightly/dev)."] # [doc = ""] # [doc = " `None` if not built via bootstrap."] pub release_channel : Option < & 'static str > , # [doc = " Information about the Git repository we may have been built from."] # [doc = ""] # [doc = " `None` if not built from a git repo."] pub commit_info : Option < CommitInfo > , }
};
}
