// Generated macro for impl_2870 (impl)
macro_rules! Depcrate_pg_types_date_and_time_quickcheck_implsimpl_2870 {
() => {
// Module: crate::pg::types::date_and_time::quickcheck_impls
// Provides: {"impl_2870"}
// Dependencies: {}
impl Arbitrary for PgInterval { fn arbitrary (g : & mut Gen) -> Self { PgInterval { microseconds : i64 :: arbitrary (g) , days : i32 :: arbitrary (g) , months : i32 :: arbitrary (g) , } } }
};
}
