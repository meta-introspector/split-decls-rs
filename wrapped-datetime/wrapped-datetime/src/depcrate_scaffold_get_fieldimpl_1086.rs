// Generated macro for impl_1086 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1086 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1086"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < YearInfo > for ZonedDateTime < A , Z > { # [inline] fn get_field (& self) -> YearInfo { self . date . year () } }
};
}
