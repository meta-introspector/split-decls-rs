// Generated macro for impl_3993 (impl)
macro_rules! Depcrate_sqlite_types_date_and_timeimpl_3993 {
() => {
// Module: crate::sqlite::types::date_and_time
// Provides: {"impl_3993"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl FromSql < sql_types :: Time , Sqlite > for String { fn from_sql (value : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Self > { FromSql :: < sql_types :: Text , Sqlite > :: from_sql (value) } }
};
}
