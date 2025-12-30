// Generated macro for impl_2868 (impl)
macro_rules! Depcrate_pg_types_date_and_time_quickcheck_implsimpl_2868 {
() => {
// Module: crate::pg::types::date_and_time::quickcheck_impls
// Provides: {"impl_2868"}
// Dependencies: {}
impl Arbitrary for PgTime { fn arbitrary (g : & mut Gen) -> Self { const MAX_TIME : i64 = 24 * 60 * 60 * 1_000_000 ; let time = u64 :: arbitrary (g) ; let mut time = if time > i64 :: MAX as u64 { (time / 2) as i64 } else { time as i64 } ; if time > MAX_TIME { time %= MAX_TIME ; } PgTime (time) } }
};
}
