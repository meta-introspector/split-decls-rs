// Generated macro for impl_1095 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1095 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1095"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < Nanosecond > for ZonedDateTime < A , Z > { # [inline] fn get_field (& self) -> Nanosecond { self . time . subsecond } }
};
}
