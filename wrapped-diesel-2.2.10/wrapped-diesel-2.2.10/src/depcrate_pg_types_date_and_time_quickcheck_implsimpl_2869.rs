// Generated macro for impl_2869 (impl)
macro_rules! Depcrate_pg_types_date_and_time_quickcheck_implsimpl_2869 {
() => {
// Module: crate::pg::types::date_and_time::quickcheck_impls
// Provides: {"impl_2869"}
// Dependencies: {}
impl Arbitrary for PgTimestamp { fn arbitrary (g : & mut Gen) -> Self { const MIN_TIMESTAMP : i64 = - 4713 * 365 * 24 * 60 * 60 * 100_000 ; const MAX_TIMESTAMP : i64 = 294276 * 365 * 24 * 60 * 60 * 100_000 ; let mut timestamp = i64 :: arbitrary (g) ; if timestamp <= MIN_TIMESTAMP { timestamp %= MIN_TIMESTAMP ; } if timestamp >= MAX_TIMESTAMP { timestamp %= MAX_TIMESTAMP ; } PgTimestamp (timestamp) } }
};
}
