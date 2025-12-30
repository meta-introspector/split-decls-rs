// Generated macro for impl_1081 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1081 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1081"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > > GetField < Hour > for DateTime < A > { # [inline] fn get_field (& self) -> Hour { self . time . hour } }
};
}
