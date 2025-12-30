// Generated macro for test_first_day (function)
macro_rules! Depcrate_weektest_first_day {
() => {
// Module: crate::week
// Provides: {"test_first_day"}
// Dependencies: {}
# [test] fn test_first_day () { use icu_locale_core :: locale ; assert_eq ! (WeekInformation :: try_new (locale ! ("und-US") . into ()) . unwrap () . first_weekday , Weekday :: Sunday ,) ; assert_eq ! (WeekInformation :: try_new (locale ! ("und-FR") . into ()) . unwrap () . first_weekday , Weekday :: Monday ,) ; assert_eq ! (WeekInformation :: try_new (locale ! ("und-FR-u-fw-tue") . into ()) . unwrap () . first_weekday , Weekday :: Tuesday ,) ; }
};
}
