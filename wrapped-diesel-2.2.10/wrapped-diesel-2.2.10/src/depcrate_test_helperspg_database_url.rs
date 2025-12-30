// Generated macro for pg_database_url (function)
macro_rules! Depcrate_test_helperspg_database_url {
() => {
// Module: crate::test_helpers
// Provides: {"pg_database_url"}
// Dependencies: {}
# [cfg (feature = "postgres")] pub fn pg_database_url () -> String { dotenvy :: var ("PG_DATABASE_URL") . or_else (| _ | dotenvy :: var ("DATABASE_URL")) . expect ("DATABASE_URL must be set in order to run tests") }
};
}
