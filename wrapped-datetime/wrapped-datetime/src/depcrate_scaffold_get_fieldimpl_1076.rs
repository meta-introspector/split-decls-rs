// Generated macro for impl_1076 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1076 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1076"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > > GetField < MonthInfo > for DateTime < A > { # [inline] fn get_field (& self) -> MonthInfo { self . date . month () } }
};
}
