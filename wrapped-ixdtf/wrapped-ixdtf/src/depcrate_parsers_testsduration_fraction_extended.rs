// Generated macro for duration_fraction_extended (function)
macro_rules! Depcrate_parsers_testsduration_fraction_extended {
() => {
// Module: crate::parsers::tests
// Provides: {"duration_fraction_extended"}
// Dependencies: {}
# [test] # [cfg (feature = "duration")] fn duration_fraction_extended () { use crate :: { parsers :: IsoDurationParser , records :: { DurationParseRecord , Sign , TimeDurationRecord } , } ; let test = "PT1H1.123456789123M" ; let result = IsoDurationParser :: from_str (test) . parse () ; assert_eq ! (result , Ok (DurationParseRecord { sign : Sign :: Positive , date : None , time : Some (TimeDurationRecord :: Minutes { hours : 1 , minutes : 1 , fraction : Some (Fraction { digits : NonZeroU8 :: new (12) . unwrap () , value : 123_456_789_123 , }) }) })) ; }
};
}
