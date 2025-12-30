// Generated macro for impl_973 (impl)
macro_rules! Depcrate_scaffold_calendarimpl_973 {
() => {
// Module: crate::scaffold::calendar
// Provides: {"impl_973"}
// Dependencies: {}
impl < C : IntoAnyCalendar , A : AsCalendar < Calendar = C > > InSameCalendar for Date < A > { # [inline] fn check_any_calendar_kind (& self , any_calendar_kind : AnyCalendarKind ,) -> Result < () , MismatchedCalendarError > { if self . calendar () . kind () == any_calendar_kind { Ok (()) } else { Err (MismatchedCalendarError { this_kind : any_calendar_kind , date_kind : Some (self . calendar () . kind ()) , }) } } }
};
}
