// Generated macro for impl_2898 (impl)
macro_rules! Depcrate_pg_types_date_and_time_timeimpl_2898 {
() => {
// Module: crate::pg::types::date_and_time::time
// Provides: {"impl_2898"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "postgres_backend"))] impl FromSql < Timestamptz , Pg > for OffsetDateTime { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let primitive_date_time = < PrimitiveDateTime as FromSql < Timestamptz , Pg > > :: from_sql (bytes) ? ; Ok (primitive_date_time . assume_utc ()) } }
};
}
