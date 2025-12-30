// Generated macro for impl_71 (impl)
macro_rules! Depcrate_postgresimpl_71 {
() => {
// Module: crate::postgres
// Provides: {"impl_71"}
// Dependencies: {}
impl FromSql < sql_types :: Time , Pg > for Time { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Time > { let PgTime (micros) = FromSql :: < sql_types :: Time , Pg > :: from_sql (bytes) ? ; let micros = jiff :: SignedDuration :: from_micros (micros) ; Ok (MIDNIGHT . checked_add (micros) ? . to_diesel ()) } }
};
}
