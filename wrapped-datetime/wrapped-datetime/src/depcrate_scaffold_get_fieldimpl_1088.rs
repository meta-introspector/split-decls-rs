// Generated macro for impl_1088 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1088 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1088"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < DayOfMonth > for ZonedDateTime < A , Z > { # [inline] fn get_field (& self) -> DayOfMonth { self . date . day_of_month () } }
};
}
