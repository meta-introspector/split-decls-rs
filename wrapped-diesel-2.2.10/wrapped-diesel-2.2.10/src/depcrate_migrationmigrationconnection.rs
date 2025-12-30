// Generated macro for MigrationConnection (trait)
macro_rules! Depcrate_migrationMigrationConnection {
() => {
// Module: crate::migration
// Provides: {"MigrationConnection"}
// Dependencies: {}
# [doc = " A trait indicating that a connection could be used to manage migrations"] # [doc = ""] # [doc = " Only custom backend implementations need to think about this trait"] pub trait MigrationConnection : Connection { # [doc = " Setup the following table:"] # [doc = ""] # [doc = " ```rust"] # [doc = " diesel::table! {"] # [doc = "      __diesel_schema_migrations(version) {"] # [doc = "          version -> Text,"] # [doc = "          /// defaults to `CURRENT_TIMESTAMP`"] # [doc = "          run_on -> Timestamp,"] # [doc = "      }"] # [doc = " }"] # [doc = " ```"] fn setup (& mut self) -> QueryResult < usize > ; }
};
}
