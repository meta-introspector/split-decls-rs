// Generated macro for impl_65 (impl)
macro_rules! Depcrate_postgresimpl_65 {
() => {
// Module: crate::postgres
// Provides: {"impl_65"}
// Dependencies: {}
impl FromSql < sql_types :: Timestamptz , Pg > for Timestamp { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Timestamp > { let PgTimestamp (micros) = FromSql :: < sql_types :: Timestamptz , Pg > :: from_sql (bytes) ? ; let micros = jiff :: SignedDuration :: from_micros (micros) ; let epoch = jiff :: Timestamp :: from_second (POSTGRES_EPOCH_TIMESTAMP) . unwrap () ; Ok (epoch . checked_add (micros) ? . to_diesel ()) } }
};
}
