// Generated macro for impl_2183 (impl)
macro_rules! Depcrate_migrationimpl_2183 {
() => {
// Module: crate::migration
// Provides: {"impl_2183"}
// Dependencies: {}
# [cfg (feature = "postgres")] impl MigrationConnection for crate :: pg :: PgConnection { fn setup (& mut self) -> QueryResult < usize > { use crate :: RunQueryDsl ; crate :: sql_query (CREATE_MIGRATIONS_TABLE) . execute (self) } }
};
}
