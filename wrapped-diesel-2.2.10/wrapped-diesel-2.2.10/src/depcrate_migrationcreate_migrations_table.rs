// Generated macro for CREATE_MIGRATIONS_TABLE (const)
macro_rules! Depcrate_migrationCREATE_MIGRATIONS_TABLE {
() => {
// Module: crate::migration
// Provides: {"CREATE_MIGRATIONS_TABLE"}
// Dependencies: {}
# [doc = " Create table statement for the `__diesel_schema_migrations` used"] # [doc = " used by the postgresql, sqlite and mysql backend"] pub const CREATE_MIGRATIONS_TABLE : & str = include_str ! ("setup_migration_table.sql") ;
};
}
