// Generated macro for impl_1098 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1098 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1098"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < ZoneNameTimestamp > for ZonedDateTime < A , Z > where Z : GetField < ZoneNameTimestamp > , { # [inline] fn get_field (& self) -> ZoneNameTimestamp { self . zone . get_field () } }
};
}
