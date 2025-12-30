// Generated macro for impl_2921 (impl)
macro_rules! Depcrate_pg_types_date_and_timeimpl_2921 {
() => {
// Module: crate::pg::types::date_and_time
// Provides: {"impl_2921"}
// Dependencies: {}
impl Add < PgInterval > for PgInterval { type Output = PgInterval ; fn add (self , other : PgInterval) -> Self :: Output { PgInterval { microseconds : self . microseconds + other . microseconds , days : self . days + other . days , months : self . months + other . months , } } }
};
}
