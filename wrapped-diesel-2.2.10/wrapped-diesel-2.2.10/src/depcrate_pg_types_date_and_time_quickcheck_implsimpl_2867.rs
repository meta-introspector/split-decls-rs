// Generated macro for impl_2867 (impl)
macro_rules! Depcrate_pg_types_date_and_time_quickcheck_implsimpl_2867 {
() => {
// Module: crate::pg::types::date_and_time::quickcheck_impls
// Provides: {"impl_2867"}
// Dependencies: {}
impl Arbitrary for PgDate { fn arbitrary (g : & mut Gen) -> Self { const MIN_DAY : i32 = (- 4713 * 365) - (2000 * 365) ; const MAX_DAY : i32 = 5874897 * 365 - (2000 * 365) ; let mut day = i32 :: arbitrary (g) ; if day <= MIN_DAY { day %= MIN_DAY ; } if day >= MAX_DAY { day %= MAX_DAY ; } PgDate (day) } }
};
}
