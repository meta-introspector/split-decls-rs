// Generated macro for impl_1077 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1077 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1077"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > > GetField < DayOfMonth > for DateTime < A > { # [inline] fn get_field (& self) -> DayOfMonth { self . date . day_of_month () } }
};
}
