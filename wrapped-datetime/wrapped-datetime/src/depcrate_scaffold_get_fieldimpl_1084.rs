// Generated macro for impl_1084 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1084 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1084"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > > GetField < Nanosecond > for DateTime < A > { # [inline] fn get_field (& self) -> Nanosecond { self . time . subsecond } }
};
}
