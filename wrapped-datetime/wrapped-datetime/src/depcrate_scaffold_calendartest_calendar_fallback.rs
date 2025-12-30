// Generated macro for test_calendar_fallback (function)
macro_rules! Depcrate_scaffold_calendartest_calendar_fallback {
() => {
// Module: crate::scaffold::calendar
// Provides: {"test_calendar_fallback"}
// Dependencies: {}
# [test] fn test_calendar_fallback () { use icu_locale_core :: locale ; assert_eq ! (FormattableAnyCalendarKind :: from_preferences (locale ! ("en-TH-u-ca-iso8601") . into ()) , FormattableAnyCalendarKind :: Buddhist) ; assert_eq ! (FormattableAnyCalendarKind :: from_preferences (locale ! ("en-TH") . into ()) , FormattableAnyCalendarKind :: Buddhist) ; assert_eq ! (FormattableAnyCalendarKind :: from_preferences (locale ! ("en-SA-u-ca-islamic") . into ()) , FormattableAnyCalendarKind :: HijriUmmAlQura) ; assert_eq ! (FormattableAnyCalendarKind :: from_preferences (locale ! ("en-IL-u-ca-islamic") . into ()) , FormattableAnyCalendarKind :: HijriTabularTypeIIFriday) ; }
};
}
