// Generated macro for impl_1090 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1090 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1090"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < DayOfYear > for ZonedDateTime < A , Z > { # [inline] fn get_field (& self) -> DayOfYear { self . date . day_of_year () } }
};
}
