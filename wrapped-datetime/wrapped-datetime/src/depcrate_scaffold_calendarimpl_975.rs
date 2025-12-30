// Generated macro for impl_975 (impl)
macro_rules! Depcrate_scaffold_calendarimpl_975 {
() => {
// Module: crate::scaffold::calendar
// Provides: {"impl_975"}
// Dependencies: {}
impl < C : IntoAnyCalendar , A : AsCalendar < Calendar = C > > InSameCalendar for DateTime < A > { # [inline] fn check_any_calendar_kind (& self , any_calendar_kind : AnyCalendarKind ,) -> Result < () , MismatchedCalendarError > { self . date . check_any_calendar_kind (any_calendar_kind) } }
};
}
