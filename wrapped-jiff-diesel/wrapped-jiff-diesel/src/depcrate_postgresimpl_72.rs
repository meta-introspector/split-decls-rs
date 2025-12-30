// Generated macro for impl_72 (impl)
macro_rules! Depcrate_postgresimpl_72 {
() => {
// Module: crate::postgres
// Provides: {"impl_72"}
// Dependencies: {}
impl FromSql < sql_types :: Interval , Pg > for Span { fn from_sql (bytes : PgValue < '_ >) -> deserialize :: Result < Span > { let interval : PgInterval = FromSql :: < sql_types :: Interval , Pg > :: from_sql (bytes) ? ; let span = jiff :: Span :: new () . try_months (interval . months) ? . try_days (interval . days) ? . try_microseconds (interval . microseconds) ? ; Ok (span . to_diesel ()) } }
};
}
