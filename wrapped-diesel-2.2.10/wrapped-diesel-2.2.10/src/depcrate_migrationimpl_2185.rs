// Generated macro for impl_2185 (impl)
macro_rules! Depcrate_migrationimpl_2185 {
() => {
// Module: crate::migration
// Provides: {"impl_2185"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl MigrationConnection for crate :: sqlite :: SqliteConnection { fn setup (& mut self) -> QueryResult < usize > { use crate :: RunQueryDsl ; crate :: sql_query (CREATE_MIGRATIONS_TABLE) . execute (self) } }
};
}
