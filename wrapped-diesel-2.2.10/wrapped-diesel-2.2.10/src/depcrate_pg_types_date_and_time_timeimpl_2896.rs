// Generated macro for impl_2896 (impl)
macro_rules! Depcrate_pg_types_date_and_time_timeimpl_2896 {
() => {
// Module: crate::pg::types::date_and_time::time
// Provides: {"impl_2896"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "postgres_backend"))] impl FromSql < Timestamptz , Pg > for PrimitiveDateTime { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { FromSql :: < Timestamp , Pg > :: from_sql (bytes) } }
};
}
