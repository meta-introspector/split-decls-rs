// Generated macro for impl_2914 (impl)
macro_rules! Depcrate_pg_types_date_and_timeimpl_2914 {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"impl_2914"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Timestamptz , Pg > for PgTimestamp { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { FromSql :: < sql_types :: Timestamp , Pg > :: from_sql (bytes) } }
};
}
