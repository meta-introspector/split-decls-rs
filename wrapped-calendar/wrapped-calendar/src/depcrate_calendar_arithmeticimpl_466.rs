// Generated macro for impl_466 (impl)
macro_rules! Depcrate_calendar_arithmeticimpl_466 {
() => {
// Module: crate::calendar_arithmetic
// Provides: {"impl_466"}
// Dependencies: {}
impl < C : DateFieldsResolver > PartialOrd for ArithmeticDate < C > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
};
}
