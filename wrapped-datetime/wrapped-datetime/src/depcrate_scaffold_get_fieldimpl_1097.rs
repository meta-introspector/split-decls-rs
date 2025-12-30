// Generated macro for impl_1097 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1097 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1097"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < TimeZone > for ZonedDateTime < A , Z > where Z : GetField < TimeZone > , { # [inline] fn get_field (& self) -> TimeZone { self . zone . get_field () } }
};
}
