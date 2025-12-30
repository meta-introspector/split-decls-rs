// Generated macro for pg_connection_no_transaction (function)
macro_rules! Depcrate_test_helperspg_connection_no_transaction {
() => {
// Module: crate::test_helpers
// Provides: {"pg_connection_no_transaction"}
// Dependencies: {}
# [cfg (feature = "postgres")] pub fn pg_connection_no_transaction () -> PgConnection { PgConnection :: establish (& pg_database_url ()) . unwrap () }
};
}
