// Generated macro for impl_2645 (impl)
macro_rules! Depcrate_pg_expression_extensions_interval_dslimpl_2645 {
() => {
// Module: crate::pg::expression::extensions::interval_dsl
// Provides: {"impl_2645"}
// Dependencies: {}
impl IntervalDsl for i32 { fn microseconds (self) -> PgInterval { i64 :: from (self) . microseconds () } fn days (self) -> PgInterval { PgInterval :: from_days (self) } fn months (self) -> PgInterval { PgInterval :: from_months (self) } fn milliseconds (self) -> PgInterval { i64 :: from (self) . milliseconds () } fn seconds (self) -> PgInterval { i64 :: from (self) . seconds () } fn minutes (self) -> PgInterval { i64 :: from (self) . minutes () } fn hours (self) -> PgInterval { i64 :: from (self) . hours () } }
};
}
