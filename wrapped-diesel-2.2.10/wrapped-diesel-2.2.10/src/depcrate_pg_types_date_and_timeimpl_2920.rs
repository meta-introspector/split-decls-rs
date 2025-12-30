// Generated macro for impl_2920 (impl)
macro_rules! Depcrate_pg_types_date_and_timeimpl_2920 {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"impl_2920"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Interval , Pg > for PgInterval { fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { Ok (PgInterval { microseconds : FromSql :: < sql_types :: BigInt , Pg > :: from_sql (value . subslice (0 .. 8)) ? , days : FromSql :: < sql_types :: Integer , Pg > :: from_sql (value . subslice (8 .. 12)) ? , months : FromSql :: < sql_types :: Integer , Pg > :: from_sql (value . subslice (12 .. 16)) ? , }) } }
};
}
