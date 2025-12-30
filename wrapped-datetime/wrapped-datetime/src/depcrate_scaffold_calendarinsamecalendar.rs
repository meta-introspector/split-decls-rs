// Generated macro for InSameCalendar (trait)
macro_rules! Depcrate_scaffold_calendarInSameCalendar {
() => {
// Module: crate::scaffold::calendar
// Provides: {"InSameCalendar"}
// Dependencies: {}
# [doc = " An input that may be associated with a specific runtime calendar."] pub trait InSameCalendar { # [doc = " Checks whether this type is compatible with the given calendar."] # [doc = ""] # [doc = " Types that are agnostic to calendar systems should return `Ok(())`."] fn check_any_calendar_kind (& self , any_calendar_kind : AnyCalendarKind ,) -> Result < () , MismatchedCalendarError > ; }
};
}
