// Generated macro for impl_2847 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2847 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2847"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl FromSql < Timestamptz , Pg > for NaiveDateTime { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { FromSql :: < Timestamp , Pg > :: from_sql (bytes) } }
};
}
