// Generated macro for RawResult (struct)
macro_rules! Depcrate_pg_connection_rawRawResult {
() => {
// Module: crate::pg::connection::raw
// Provides: {"RawResult"}
// Dependencies: {}
# [doc = " Internal wrapper around a `*mut PGresult` which is known to be not-null, and"] # [doc = " have no aliases.  This wrapper is to ensure that it's always properly"] # [doc = " dropped."] # [doc = ""] # [doc = " If `Unique` is ever stabilized, we should use it here."] # [allow (missing_debug_implementations)] pub (super) struct RawResult (NonNull < PGresult >) ;
};
}
