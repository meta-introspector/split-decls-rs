// Generated macro for impl_1083 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1083 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1083"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > > GetField < Second > for DateTime < A > { # [inline] fn get_field (& self) -> Second { self . time . second } }
};
}
