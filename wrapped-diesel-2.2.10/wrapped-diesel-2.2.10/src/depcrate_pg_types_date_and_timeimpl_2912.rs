// Generated macro for impl_2912 (impl)
macro_rules! Depcrate_pg_types_date_and_timeimpl_2912 {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"impl_2912"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Timestamp , Pg > for PgTimestamp { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { FromSql :: < sql_types :: BigInt , Pg > :: from_sql (bytes) . map (PgTimestamp) } }
};
}
