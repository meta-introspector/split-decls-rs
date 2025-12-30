// Generated macro for impl_23 (impl)
macro_rules! Depcrate_dateimpl_23 {
() => {
// Module: crate::date
// Provides: {"impl_23"}
// Dependencies: {}
impl < C : AsCalendar > AsCalendar for Ref < '_ , C > { type Calendar = C :: Calendar ; # [inline] fn as_calendar (& self) -> & Self :: Calendar { self . 0 . as_calendar () } }
};
}
