// Generated macro for impl_2862 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2862 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2862"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl FromSql < Interval , Pg > for Duration { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let interval : PgInterval = FromSql :: < Interval , Pg > :: from_sql (bytes) ? ; let days = interval . months * DAYS_PER_MONTH + interval . days ; Ok (Duration :: days (days as i64) + Duration :: microseconds (interval . microseconds)) } }
};
}
