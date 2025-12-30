// Generated macro for impl_2850 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2850 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2850"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl FromSql < Timestamptz , Pg > for DateTime < Local > { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let naive_date_time = < NaiveDateTime as FromSql < Timestamptz , Pg > > :: from_sql (bytes) ? ; Ok (Local :: from_utc_datetime (& Local , & naive_date_time)) } }
};
}
