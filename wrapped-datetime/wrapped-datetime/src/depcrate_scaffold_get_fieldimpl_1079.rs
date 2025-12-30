// Generated macro for impl_1079 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1079 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1079"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > > GetField < DayOfYear > for DateTime < A > { # [inline] fn get_field (& self) -> DayOfYear { self . date . day_of_year () } }
};
}
