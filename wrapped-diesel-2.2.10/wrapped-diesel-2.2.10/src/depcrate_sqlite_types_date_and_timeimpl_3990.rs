// Generated macro for impl_3990 (impl)
macro_rules! Depcrate_sqlite_types_date_and_timeimpl_3990 {
() => {
// Module: crate::sqlite::types::date_and_time
// Provides: {"impl_3990"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl FromSql < sql_types :: Date , Sqlite > for String { fn from_sql (value : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Self > { FromSql :: < sql_types :: Text , Sqlite > :: from_sql (value) } }
};
}
