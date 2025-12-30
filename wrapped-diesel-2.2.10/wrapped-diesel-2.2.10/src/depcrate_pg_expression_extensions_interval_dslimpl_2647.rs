// Generated macro for impl_2647 (impl)
macro_rules! Depcrate_pg_expression_extensions_interval_dslimpl_2647 {
() => {
// Module: crate::pg::expression::extensions::interval_dsl
// Provides: {"impl_2647"}
// Dependencies: {}
# [allow (clippy :: cast_possible_truncation)] impl IntervalDsl for f64 { fn microseconds (self) -> PgInterval { (self . round () as i64) . microseconds () } fn days (self) -> PgInterval { let fractional_days = (self . fract () * 86_400.0) . seconds () ; PgInterval :: from_days (self . trunc () as i32) + fractional_days } fn months (self) -> PgInterval { let fractional_months = (self . fract () * 30.0) . days () ; PgInterval :: from_months (self . trunc () as i32) + fractional_months } fn years (self) -> PgInterval { ((self * 12.0) . trunc () as i32) . months () } }
};
}
