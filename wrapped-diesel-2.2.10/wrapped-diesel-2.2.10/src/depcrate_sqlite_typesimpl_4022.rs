// Generated macro for impl_4022 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4022 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4022"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl FromSql < sql_types :: Integer , Sqlite > for i32 { fn from_sql (mut value : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Self > { Ok (value . read_integer ()) } }
};
}
