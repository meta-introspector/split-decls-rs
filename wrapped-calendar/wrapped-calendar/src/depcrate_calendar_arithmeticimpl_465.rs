// Generated macro for impl_465 (impl)
macro_rules! Depcrate_calendar_arithmeticimpl_465 {
() => {
// Module: crate::calendar_arithmetic
// Provides: {"impl_465"}
// Dependencies: {}
impl < C : DateFieldsResolver > Ord for ArithmeticDate < C > { fn cmp (& self , other : & Self) -> Ordering { self . year () . to_extended_year () . cmp (& other . year () . to_extended_year ()) . then (self . month () . cmp (& other . month ())) . then (self . day () . cmp (& other . day ())) } }
};
}
