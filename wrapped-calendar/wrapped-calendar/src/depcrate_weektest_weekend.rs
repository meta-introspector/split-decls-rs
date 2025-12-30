// Generated macro for test_weekend (function)
macro_rules! Depcrate_weektest_weekend {
() => {
// Module: crate::week
// Provides: {"test_weekend"}
// Dependencies: {}
# [test] fn test_weekend () { use icu_locale_core :: locale ; assert_eq ! (WeekInformation :: try_new (locale ! ("und") . into ()) . unwrap () . weekend () . collect ::< Vec < _ >> () , vec ! [Weekday :: Saturday , Weekday :: Sunday] ,) ; assert_eq ! (WeekInformation :: try_new (locale ! ("und-FR") . into ()) . unwrap () . weekend () . collect ::< Vec < _ >> () , vec ! [Weekday :: Saturday , Weekday :: Sunday] ,) ; assert_eq ! (WeekInformation :: try_new (locale ! ("und-IQ") . into ()) . unwrap () . weekend () . collect ::< Vec < _ >> () , vec ! [Weekday :: Saturday , Weekday :: Friday] ,) ; assert_eq ! (WeekInformation :: try_new (locale ! ("und-IR") . into ()) . unwrap () . weekend () . collect ::< Vec < _ >> () , vec ! [Weekday :: Friday] ,) ; }
};
}
