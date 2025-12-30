// Generated macro for impl_2879 (impl)
macro_rules! Depcrate_pg_types_date_and_time_std_timeimpl_2879 {
() => {
// Module: crate::pg::types::date_and_time::std_time
// Provides: {"impl_2879"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Timestamp , Pg > for SystemTime { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let usecs_passed = < i64 as FromSql < sql_types :: BigInt , Pg > > :: from_sql (bytes) ? ; let before_epoch = usecs_passed < 0 ; let time_passed = usecs_to_duration (usecs_passed . unsigned_abs ()) ; if before_epoch { Ok (pg_epoch () - time_passed) } else { Ok (pg_epoch () + time_passed) } } }
};
}
