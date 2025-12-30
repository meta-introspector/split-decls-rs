// Generated macro for PgResult (struct)
macro_rules! Depcrate_pg_connection_resultPgResult {
() => {
// Module: crate::pg::connection::result
// Provides: {"PgResult"}
// Dependencies: {}
# [allow (missing_debug_implementations)] pub struct PgResult { internal_result : RawResult , column_count : libc :: c_int , row_count : libc :: c_int , column_name_map : OnceCell < Vec < Option < * const str > > > , }
};
}
