// Generated macro for impl_1080 (impl)
macro_rules! Depcrate_scaffold_get_fieldimpl_1080 {
() => {
// Module: crate::scaffold::get_field
// Provides: {"impl_1080"}
// Dependencies: {}
impl < C : Calendar , A : AsCalendar < Calendar = C > > GetField < RataDie > for DateTime < A > { # [inline] fn get_field (& self) -> RataDie { self . date . to_rata_die () } }
};
}
