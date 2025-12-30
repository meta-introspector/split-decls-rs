// Generated macro for pg_connection (function)
macro_rules! Depcrate_test_helperspg_connection {
() => {
// Module: crate::test_helpers
// Provides: {"pg_connection"}
// Dependencies: {}
# [cfg (feature = "postgres")] pub fn pg_connection () -> PgConnection { let mut conn = pg_connection_no_transaction () ; conn . begin_test_transaction () . unwrap () ; conn }
};
}
