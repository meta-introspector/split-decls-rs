// Generated macro for impl_2646 (impl)
macro_rules! Depcrate_pg_expression_extensions_interval_dslimpl_2646 {
() => {
// Module: crate::pg::expression::extensions::interval_dsl
// Provides: {"impl_2646"}
// Dependencies: {}
impl IntervalDsl for i64 { fn microseconds (self) -> PgInterval { PgInterval :: from_microseconds (self) } fn days (self) -> PgInterval { i32 :: try_from (self) . expect ("Maximal supported day interval size is 32 bit") . days () } fn months (self) -> PgInterval { i32 :: try_from (self) . expect ("Maximal supported month interval size is 32 bit") . months () } }
};
}
