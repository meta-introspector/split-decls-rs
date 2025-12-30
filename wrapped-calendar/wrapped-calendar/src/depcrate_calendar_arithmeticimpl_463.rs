// Generated macro for impl_463 (impl)
macro_rules! Depcrate_calendar_arithmeticimpl_463 {
() => {
// Module: crate::calendar_arithmetic
// Provides: {"impl_463"}
// Dependencies: {}
impl < C : DateFieldsResolver > PartialEq for ArithmeticDate < C > { fn eq (& self , other : & Self) -> bool { self . year () . to_extended_year () == other . year () . to_extended_year () && self . month () == other . month () && self . day () == other . day () } }
};
}
