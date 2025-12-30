// Generated macro for IntoFormattableAnyCalendar (trait)
macro_rules! Depcrate_scaffold_calendarIntoFormattableAnyCalendar {
() => {
// Module: crate::scaffold::calendar
// Provides: {"IntoFormattableAnyCalendar"}
// Dependencies: {}
# [doc = " A calendar type that is supported by [`DateTimeFormatter`](crate::DateTimeFormatter)."] # [doc = ""] # [doc = " [`FixedCalendarDateTimeFormatter`](crate::FixedCalendarDateTimeFormatter) might support additional calendars."] pub trait IntoFormattableAnyCalendar : CldrCalendar + IntoAnyCalendar { }
};
}
