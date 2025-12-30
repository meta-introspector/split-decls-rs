// Generated macro for AsCalendar (trait)
macro_rules! Depcrate_dateAsCalendar {
() => {
// Module: crate::date
// Provides: {"AsCalendar"}
// Dependencies: {}
# [doc = " Types that contain a calendar"] # [doc = ""] # [doc = " This allows one to use [`Date`] with wrappers around calendars,"] # [doc = " e.g. reference counted calendars."] pub trait AsCalendar { # [doc = " The calendar being wrapped"] type Calendar : Calendar ; # [doc = " Obtain the inner calendar"] fn as_calendar (& self) -> & Self :: Calendar ; }
};
}
