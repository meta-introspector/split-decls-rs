// Generated macro for impl_2849 (impl)
macro_rules! Depcrate_pg_types_date_and_time_chronoimpl_2849 {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"impl_2849"}
// Dependencies: {}
# [cfg (all (feature = "chrono" , feature = "postgres_backend"))] impl FromSql < Timestamptz , Pg > for DateTime < Utc > { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Self > { let naive_date_time = < NaiveDateTime as FromSql < Timestamptz , Pg > > :: from_sql (bytes) ? ; Ok (Utc . from_utc_datetime (& naive_date_time)) } }
};
}
