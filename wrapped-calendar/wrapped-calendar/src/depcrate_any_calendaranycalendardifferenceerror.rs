// Generated macro for AnyCalendarDifferenceError (enum)
macro_rules! Depcrate_any_calendarAnyCalendarDifferenceError {
() => {
// Module: crate::any_calendar
// Provides: {"AnyCalendarDifferenceError"}
// Dependencies: {}
# [doc = " Error returned when comparing two [`Date`]s with [`AnyCalendar`]."] # [derive (Clone , Copy , PartialEq , Debug)] # [non_exhaustive] # [doc (hidden)] pub enum AnyCalendarDifferenceError { # [doc = " The calendars of the two dates being compared are not equal."] # [doc = ""] # [doc = " To compare dates in different calendars, convert them to the same calendar first."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::calendar::cal::AnyCalendarDifferenceError;"] # [doc = " use icu::calendar::Date;"] # [doc = ""] # [doc = " let d1 = Date::try_new_gregorian(2000, 1, 1).unwrap().to_any();"] # [doc = " let d2 = Date::try_new_persian(1562, 1, 1).unwrap().to_any();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     d1.try_until_with_options(&d2, Default::default())"] # [doc = "         .unwrap_err(),"] # [doc = "     AnyCalendarDifferenceError::MismatchedCalendars,"] # [doc = " );"] # [doc = ""] # [doc = " // To compare the dates, convert them to the same calendar,"] # [doc = " // such as ISO."] # [doc = ""] # [doc = " d1.to_iso()"] # [doc = "     .try_until_with_options(&d2.to_iso(), Default::default())"] # [doc = "     .unwrap();"] # [doc = " ```"] MismatchedCalendars , }
};
}
