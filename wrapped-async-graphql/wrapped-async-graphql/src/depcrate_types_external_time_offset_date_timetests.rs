// Generated macro for tests (module)
macro_rules! Depcrate_types_external_time_offset_date_timetests {
() => {
// Module: crate::types::external::time_offset_date_time
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use time :: { OffsetDateTime , macros :: datetime } ; use crate :: { ScalarType , Value } ; # [test] fn test_offset_date_time_to_value () { let cases = [(datetime ! (2022 - 01 - 12 07 : 30 : 19.12345 + 3 : 30) , "2022-01-12T04:00:19.12345Z" ,) , (datetime ! (2022 - 01 - 12 07 : 30 : 19 - 0) , "2022-01-12T07:30:19Z") ,] ; for (value , expected) in cases { let value = value . to_value () ; if let Value :: String (s) = value { assert_eq ! (s , expected) ; } else { panic ! ("Unexpected Value type when formatting OffsetDateTime: {:?}" , value) ; } } } # [test] fn test_offset_date_time_parse () { let cases = [("2022-01-12T04:00:19.12345Z" , datetime ! (2022 - 01 - 12 07 : 30 : 19.12345 + 3 : 30) ,) , ("2022-01-12T23:22:19.12345-00:00" , datetime ! (2022 - 01 - 12 23 : 22 : 19.12345 - 0) ,) ,] ; for (value , expected) in cases { let value = Value :: String (value . to_string ()) ; let parsed = < OffsetDateTime as ScalarType > :: parse (value) . unwrap () ; assert_eq ! (parsed , expected) ; } } }
};
}
