// Generated macro for temporal_duration_parsing (function)
macro_rules! Depcrate_parsers_teststemporal_duration_parsing {
() => {
// Module: crate::parsers::tests
// Provides: {"temporal_duration_parsing"}
// Dependencies: {}
# [test] # [cfg (feature = "duration")] fn temporal_duration_parsing () { use crate :: { parsers :: IsoDurationParser , records :: { DateDurationRecord , DurationParseRecord , Sign , TimeDurationRecord } , } ; let durations = ["p1y1m1dt1h1m1s" , "P1Y1M1W1DT1H1M1.1S" , "-P1Y1M1W1DT1H1M1.123456789S" , "-P1Y3wT0,5H" ,] ; for dur in durations { let ok_result = IsoDurationParser :: from_str (dur) . parse () ; assert ! (ok_result . is_ok () , "Failing to parse a valid ISO 8601 target: \"{dur}\" should pass.") ; } let sub_second = IsoDurationParser :: from_str (durations [2]) . parse () . unwrap () ; assert_eq ! (sub_second , DurationParseRecord { sign : Sign :: Negative , date : Some (DateDurationRecord { years : 1 , months : 1 , weeks : 1 , days : 1 , }) , time : Some (TimeDurationRecord :: Seconds { hours : 1 , minutes : 1 , seconds : 1 , fraction : Some (Fraction { digits : NonZeroU8 :: new (9) . unwrap () , value : 123456789 , }) }) } , "Failing to parse a valid Duration string: \"{}\" should pass." , durations [2]) ; let test_result = IsoDurationParser :: from_str (durations [3]) . parse () . unwrap () ; assert_eq ! (test_result , DurationParseRecord { sign : Sign :: Negative , date : Some (DateDurationRecord { years : 1 , months : 0 , weeks : 3 , days : 0 , }) , time : Some (TimeDurationRecord :: Hours { hours : 0 , fraction : Some (Fraction { digits : NonZeroU8 :: new (1) . unwrap () , value : 5 }) , }) }) ; }
};
}
