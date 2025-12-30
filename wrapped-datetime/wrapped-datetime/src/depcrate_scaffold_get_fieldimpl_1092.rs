// Generated macro for impl_1092 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1092 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1092"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < Hour > for ZonedDateTime < A , Z > { # [inline] fn get_field (& self) -> Hour { self . time . hour } }
};
}
