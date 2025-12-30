// Generated macro for good_annotations_date_time (function)
macro_rules! Depcrate_parsers_testsgood_annotations_date_time {
() => {
// Module: crate::parsers::tests
// Provides: {"good_annotations_date_time"}
// Dependencies: {}
# [test] fn good_annotations_date_time () { let result = IxdtfParser :: from_str ("2020-11-08[!America/Argentina/ComodRivadavia][u-ca=iso8601][foo=bar]" ,) . parse () . unwrap () ; let tz_annotation = result . tz . unwrap () ; assert_eq ! (tz_annotation , TimeZoneAnnotation { critical : true , tz : TimeZoneRecord :: Name ("America/Argentina/ComodRivadavia" . as_bytes ()) , }) ; assert_eq ! (result . calendar , Some ("iso8601" . as_bytes ())) ; let omit_result = IxdtfParser :: from_str ("+0020201108[!u-ca=iso8601][f-1a2b=a0sa-2l4s]") . parse () . unwrap () ; assert ! (omit_result . tz . is_none ()) ; assert_eq ! (omit_result . calendar , Some ("iso8601" . as_bytes ())) ; }
};
}
