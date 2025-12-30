// Generated macro for impl_1078 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1078 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1078"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > > GetField < Weekday > for DateTime < A > { # [inline] fn get_field (& self) -> Weekday { self . date . day_of_week () } }
};
}
