// Generated macro for impl_1075 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1075 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1075"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > > GetField < YearInfo > for DateTime < A > { # [inline] fn get_field (& self) -> YearInfo { self . date . year () } }
};
}
