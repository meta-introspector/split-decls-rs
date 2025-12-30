// Generated macro for impl_1066 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1066 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1066"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > > GetField < Weekday > for Date < A > { # [inline] fn get_field (& self) -> Weekday { self . day_of_week () } }
};
}
