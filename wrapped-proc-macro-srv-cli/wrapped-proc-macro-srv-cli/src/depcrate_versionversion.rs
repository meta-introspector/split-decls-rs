// Generated macro for version (function)
macro_rules! Depcrate_versionversion {
() => {
// Module: crate::version
// Provides: {"version"}
// Dependencies: {}
# [doc = " Returns information about cargo's version."] pub (crate) const fn version () -> VersionInfo { let version = match option_env ! ("CFG_RELEASE") { Some (x) => x , None => "0.0.0" , } ; let release_channel = option_env ! ("CFG_RELEASE_CHANNEL") ; let commit_info = match (option_env ! ("RA_COMMIT_SHORT_HASH") , option_env ! ("RA_COMMIT_HASH") , option_env ! ("RA_COMMIT_DATE") ,) { (Some (short_commit_hash) , Some (commit_hash) , Some (commit_date)) => { Some (CommitInfo { short_commit_hash , commit_hash , commit_date }) } _ => None , } ; VersionInfo { version , release_channel , commit_info } }
};
}
