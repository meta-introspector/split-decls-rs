// Generated macro for impl_1093 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1093 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1093"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < Minute > for ZonedDateTime < A , Z > { # [inline] fn get_field (& self) -> Minute { self . time . minute } }
};
}
