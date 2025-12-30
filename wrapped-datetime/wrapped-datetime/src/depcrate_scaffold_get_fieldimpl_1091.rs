// Generated macro for impl_1091 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1091 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1091"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > , Z > GetField < RataDie > for ZonedDateTime < A , Z > { # [inline] fn get_field (& self) -> RataDie { self . date . to_rata_die () } }
};
}
