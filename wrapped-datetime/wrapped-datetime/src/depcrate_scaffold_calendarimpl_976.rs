// Generated macro for impl_976 (impl)
macro_rules! Depcrate_scaffold_calendarimpl_976 {
() => {
// Module: crate::scaffold::calendar
// Provides: {"impl_976"}
// Dependencies: {}
impl < C : IntoAnyCalendar , A : AsCalendar < Calendar = C > , Z > InSameCalendar for ZonedDateTime < A , Z > { # [inline] fn check_any_calendar_kind (& self , any_calendar_kind : AnyCalendarKind ,) -> Result < () , MismatchedCalendarError > { self . date . check_any_calendar_kind (any_calendar_kind) } }
};
}
