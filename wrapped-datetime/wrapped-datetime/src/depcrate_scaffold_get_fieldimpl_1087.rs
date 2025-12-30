// Generated macro for impl_1087 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1087 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1087"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < MonthInfo > for ZonedDateTime < A , Z > { # [inline] fn get_field (& self) -> MonthInfo { self . date . month () } }
};
}
