// Generated macro for impl_4024 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4024 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4024"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl FromSql < sql_types :: BigInt , Sqlite > for i64 { fn from_sql (mut value : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Self > { Ok (value . read_long ()) } }
};
}
