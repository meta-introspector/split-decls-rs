// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use jiff :: ToSpan ; use super :: * ; # [doc = " This exhaustively confirms that all valid Jiff dates are also valid"] # [doc = " ICU4X dates."] # [doc = ""] # [doc = " I believe the reverse is not true, although it's not quite clear from"] # [doc = " ICU4X's docs. (Although I haven't done an exhaustive search.)"] # [doc = ""] # [doc = " This test is ignored because it takes forever in debug mode (sigh). In"] # [doc = " release mode it is quite snappy. But I have run it. And the doc tests"] # [doc = " above check the min and max values."] # [test] # [ignore] fn all_jiff_dates_are_valid_icu_dates () { for jiff_date in jiff :: civil :: Date :: MIN . series (1 . day ()) { let icu_date : IcuDate < Iso > = jiff_date . convert_try_into () . unwrap () ; let got : jiff :: civil :: Date = icu_date . convert_try_into () . unwrap () ; assert_eq ! (jiff_date , got) ; } } # [doc = " Like the above, but for civil times."] # [doc = ""] # [doc = " We skip nanoseconds because it would take too long to exhaustively"] # [doc = " test. Without nanoseconds, this is quick enough to run in debug mode."] # [cfg (feature = "time")] # [test] fn all_jiff_times_are_valid_icu_times () { for jiff_time in jiff :: civil :: Time :: MIN . series (1 . second ()) { let icu_time : IcuTime = jiff_time . convert_try_into () . unwrap () ; let got : jiff :: civil :: Time = icu_time . convert_try_into () . unwrap () ; assert_eq ! (jiff_time , got) ; } } }
};
}
