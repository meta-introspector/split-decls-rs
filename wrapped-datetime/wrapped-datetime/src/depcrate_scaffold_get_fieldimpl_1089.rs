// Generated macro for impl_1089 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1089 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1089"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < Weekday > for ZonedDateTime < A , Z > { # [inline] fn get_field (& self) -> Weekday { self . date . day_of_week () } }
};
}
