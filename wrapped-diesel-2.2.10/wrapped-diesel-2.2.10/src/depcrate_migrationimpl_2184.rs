// Generated macro for impl_2184 (impl)
macro_rules! Depcrate_migrationimpl_2184 {
() => {
// Module: crate::migration
// Provides: {"impl_2184"}
// Dependencies: {}
# [cfg (feature = "mysql")] impl MigrationConnection for crate :: mysql :: MysqlConnection { fn setup (& mut self) -> QueryResult < usize > { use crate :: RunQueryDsl ; crate :: sql_query (CREATE_MIGRATIONS_TABLE) . execute (self) } }
};
}
