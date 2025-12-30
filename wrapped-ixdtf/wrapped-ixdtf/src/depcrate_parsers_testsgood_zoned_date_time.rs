// Generated macro for good_zoned_date_time (function)
macro_rules! Depcrate_parsers_testsgood_zoned_date_time {
() => {
// Module: crate::parsers::tests
// Provides: {"good_zoned_date_time"}
// Dependencies: {}
# [test] fn good_zoned_date_time () { let result = IxdtfParser :: from_str ("2020-04-08[America/Chicago]") . parse () . unwrap () ; assert_eq ! (result . date , Some (DateRecord { year : 2020 , month : 4 , day : 8 , })) ; let tz_annotation = result . tz . unwrap () ; assert_eq ! (tz_annotation , TimeZoneAnnotation { critical : false , tz : TimeZoneRecord :: Name ("America/Chicago" . as_bytes ()) }) ; }
};
}
