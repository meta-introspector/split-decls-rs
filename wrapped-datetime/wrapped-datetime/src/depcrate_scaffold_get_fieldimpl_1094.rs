// Generated macro for impl_1094 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1094 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1094"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < Second > for ZonedDateTime < A , Z > { # [inline] fn get_field (& self) -> Second { self . time . second } }
};
}
