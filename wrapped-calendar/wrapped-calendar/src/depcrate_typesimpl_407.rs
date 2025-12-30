// Generated macro for impl_407 (impl)
macro_rules! Depcrate_typesimpl_407 {
() => {
// Module: crate::types
// Provides: {"impl_407"}
// Dependencies: {}
impl From < DayOfMonth > for DayOfWeekInMonth { fn from (day_of_month : DayOfMonth) -> Self { DayOfWeekInMonth (1 + ((day_of_month . 0 - 1) / 7)) } }
};
}
