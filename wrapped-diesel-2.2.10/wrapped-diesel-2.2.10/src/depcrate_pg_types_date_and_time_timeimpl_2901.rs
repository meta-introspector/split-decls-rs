// Generated macro for impl_2901 (impl)
macro_rules! Depcrate_pg_types_date_and_time_timeimpl_2901 {
() => {
// Module: crate::pg::types::date_and_time::time
// Provides: {"impl_2901"}
// Dependencies: {}
# [cfg (all (feature = "time" , feature = "postgres_backend"))] impl FromSql < Time , Pg > for NaiveTime { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let PgTime (offset) = FromSql :: < Time , Pg > :: from_sql (bytes) ? ; let duration = Duration :: microseconds (offset) ; Ok (NaiveTime :: MIDNIGHT + duration) } }
};
}
