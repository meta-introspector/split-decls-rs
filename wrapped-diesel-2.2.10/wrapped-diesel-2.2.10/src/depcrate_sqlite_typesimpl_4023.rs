// Generated macro for impl_4023 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4023 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4023"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl FromSql < sql_types :: Bool , Sqlite > for bool { fn from_sql (mut value : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Self > { Ok (value . read_integer () != 0) } }
};
}
