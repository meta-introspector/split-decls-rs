// Generated macro for impl_2910 (impl)
macro_rules! Depcrate_pg_types_date_and_timeimpl_2910 {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"impl_2910"}
// Dependencies: {}
impl PgInterval { # [doc = " Constructs a new `PgInterval`"] # [doc = ""] # [doc = " No conversion occurs on the arguments. It is valid to provide a number"] # [doc = " of microseconds greater than the longest possible day, or a number of"] # [doc = " days greater than the longest possible month, as it is impossible to say"] # [doc = " how many months are in \"40 days\" without knowing a precise date."] pub fn new (microseconds : i64 , days : i32 , months : i32) -> Self { PgInterval { microseconds , days , months , } } # [doc = " Equivalent to `new(microseconds, 0, 0)`"] pub fn from_microseconds (microseconds : i64) -> Self { Self :: new (microseconds , 0 , 0) } # [doc = " Equivalent to `new(0, days, 0)`"] pub fn from_days (days : i32) -> Self { Self :: new (0 , days , 0) } # [doc = " Equivalent to `new(0, 0, months)`"] pub fn from_months (months : i32) -> Self { Self :: new (0 , 0 , months) } }
};
}
