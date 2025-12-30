// Generated macro for impl_2916 (impl)
macro_rules! Depcrate_pg_types_date_and_timeimpl_2916 {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"impl_2916"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Date , Pg > for PgDate { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { FromSql :: < sql_types :: Integer , Pg > :: from_sql (bytes) . map (PgDate) } }
};
}
