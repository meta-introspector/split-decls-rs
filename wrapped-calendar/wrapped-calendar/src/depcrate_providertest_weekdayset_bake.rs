// Generated macro for test_weekdayset_bake (function)
macro_rules! Depcrate_providertest_weekdayset_bake {
() => {
// Module: crate::provider
// Provides: {"test_weekdayset_bake"}
// Dependencies: {}
# [test] # [cfg (feature = "datagen")] fn test_weekdayset_bake () { databake :: test_bake ! (WeekdaySet , const , crate :: provider :: WeekdaySet :: new (& [crate :: types :: Weekday :: Monday , crate :: types :: Weekday :: Wednesday , crate :: types :: Weekday :: Friday]) , icu_calendar) ; }
};
}
