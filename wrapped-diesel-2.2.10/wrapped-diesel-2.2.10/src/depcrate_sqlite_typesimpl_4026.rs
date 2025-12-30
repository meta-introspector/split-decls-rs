// Generated macro for impl_4026 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4026 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4026"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl FromSql < sql_types :: Double , Sqlite > for f64 { fn from_sql (mut value : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Self > { Ok (value . read_double ()) } }
};
}
