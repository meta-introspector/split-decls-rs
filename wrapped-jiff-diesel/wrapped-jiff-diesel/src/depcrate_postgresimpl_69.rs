// Generated macro for impl_69 (impl)
macro_rules! Depcrate_postgresimpl_69 {
() => {
// Module: crate::postgres
// Provides: {"impl_69"}
// Dependencies: {}
impl FromSql < sql_types :: Date , Pg > for Date { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Date > { let PgDate (days) = FromSql :: < sql_types :: Date , Pg > :: from_sql (bytes) ? ; let span = jiff :: Span :: new () . try_days (days) ? ; Ok (POSTGRES_EPOCH_DATE . checked_add (span) ? . to_diesel ()) } }
};
}
