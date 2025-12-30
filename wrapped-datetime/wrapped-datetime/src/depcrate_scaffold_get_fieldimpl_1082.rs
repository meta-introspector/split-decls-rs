// Generated macro for impl_1082 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1082 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1082"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > > GetField < Minute > for DateTime < A > { # [inline] fn get_field (& self) -> Minute { self . time . minute } }
};
}
