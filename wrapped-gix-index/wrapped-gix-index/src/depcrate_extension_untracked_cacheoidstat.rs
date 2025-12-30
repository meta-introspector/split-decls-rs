// Generated macro for OidStat (struct)
macro_rules! Depcrate_extension_untracked_cacheOidStat {
() => {
// Module: crate::extension::untracked_cache
// Provides: {"OidStat"}
// Dependencies: {}
# [doc = " A structure to track filesystem stat information along with an object id, linking a worktree file with what's in our ODB."] # [derive (Clone)] pub struct OidStat { # [doc = " The file system stat information"] pub stat : entry :: Stat , # [doc = " The id of the file in our ODB."] pub id : ObjectId , }
};
}
