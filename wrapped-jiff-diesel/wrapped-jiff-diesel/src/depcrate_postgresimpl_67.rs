// Generated macro for impl_67 (impl)
macro_rules! Depcrate_postgresimpl_67 {
() => {
// Module: crate::postgres
// Provides: {"impl_67"}
// Dependencies: {}
impl FromSql < sql_types :: Timestamp , Pg > for DateTime { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < DateTime > { let PgTimestamp (micros) = FromSql :: < sql_types :: Timestamp , Pg > :: from_sql (bytes) ? ; let micros = jiff :: SignedDuration :: from_micros (micros) ; Ok (POSTGRES_EPOCH_DATETIME . checked_add (micros) ? . to_diesel ()) } }
};
}
