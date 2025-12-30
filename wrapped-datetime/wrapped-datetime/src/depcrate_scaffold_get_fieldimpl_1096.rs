// Generated macro for impl_1096 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1096 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1096"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < Option < UtcOffset > > for ZonedDateTime < A , Z > where Z : GetField < Option < UtcOffset > > , { # [inline] fn get_field (& self) -> Option < UtcOffset > { self . zone . get_field () } }
};
}
