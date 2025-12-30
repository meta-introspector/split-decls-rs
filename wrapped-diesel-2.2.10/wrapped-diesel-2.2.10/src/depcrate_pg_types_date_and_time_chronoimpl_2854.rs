// Generated macro for impl_2854 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2854 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2854"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl FromSql < Time , Pg > for NaiveTime { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let PgTime (offset) = FromSql :: < Time , Pg > :: from_sql (bytes) ? ; let duration = Duration :: microseconds (offset) ; Ok (midnight () + duration) } }
};
}
