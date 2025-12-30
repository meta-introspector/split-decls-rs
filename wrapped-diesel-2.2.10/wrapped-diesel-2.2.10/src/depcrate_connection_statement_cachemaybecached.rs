// Generated macro for MaybeCached (enum)
macro_rules! Depcrate_connection_statement_cacheMaybeCached {
() => {
// Module: crate::connection::statement_cache
// Provides: {"MaybeCached"}
// Dependencies: {}
# [doc = " Wraps a possibly cached prepared statement"] # [doc = ""] # [doc = " Essentially a customized version of [`Cow`]"] # [doc = " that does not depend on [`ToOwned`]"] # [allow (missing_debug_implementations , unreachable_pub)] # [cfg_attr (docsrs , doc (cfg (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")))] # [non_exhaustive] pub enum MaybeCached < 'a , T : 'a > { # [doc = " Contains a not cached prepared statement"] CannotCache (T) , # [doc = " Contains a reference cached prepared statement"] Cached (& 'a mut T) , }
};
}
