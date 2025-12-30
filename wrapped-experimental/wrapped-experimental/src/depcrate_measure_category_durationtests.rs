// Generated macro for tests (module)
macro_rules! Depcrate_measure_category_durationtests {
() => {
// Module: crate::measure::category::duration
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: measure :: measureunit :: MeasureUnit ; # [test] fn test_duration_category () { let millisecond = Duration :: millisecond () ; let millisecond_parsed = MeasureUnit :: try_from_str ("millisecond") . unwrap () ; assert_eq ! (millisecond . unit , millisecond_parsed) ; let second = Duration :: second () ; let second_parsed = MeasureUnit :: try_from_str ("second") . unwrap () ; assert_eq ! (second . unit , second_parsed) ; let minute = Duration :: minute () ; let minute_parsed = MeasureUnit :: try_from_str ("minute") . unwrap () ; assert_eq ! (minute . unit , minute_parsed) ; let hour = Duration :: hour () ; let hour_parsed = MeasureUnit :: try_from_str ("hour") . unwrap () ; assert_eq ! (hour . unit , hour_parsed) ; let day = Duration :: day () ; let day_parsed = MeasureUnit :: try_from_str ("day") . unwrap () ; assert_eq ! (day . unit , day_parsed) ; let week = Duration :: week () ; let week_parsed = MeasureUnit :: try_from_str ("week") . unwrap () ; assert_eq ! (week . unit , week_parsed) ; } }
};
}
