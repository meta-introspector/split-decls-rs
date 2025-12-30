// Generated macro for impl_2796 (impl)
macro_rules! Depcrate_pg_expression_date_and_timeimpl_2796 {
() => {
// Module: crate::pg::expression::date_and_time
// Provides: {"impl_2796"}
// Dependencies: {}
impl < Ts , Tz > Expression for AtTimeZone < Ts , Tz > where Ts : Expression , Ts :: SqlType : DateTimeLike , Tz : Expression < SqlType = VarChar > , { type SqlType = Timestamp ; }
};
}
