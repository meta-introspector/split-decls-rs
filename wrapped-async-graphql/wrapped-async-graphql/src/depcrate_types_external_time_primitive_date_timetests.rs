// Generated macro for tests (module)
macro_rules! Depcrate_types_external_time_primitive_date_timetests {
() => {
// Module: crate::types::external::time_primitive_date_time
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use time :: { PrimitiveDateTime , macros :: datetime } ; use crate :: { ScalarType , Value } ; # [test] fn test_primitive_date_time_to_value () { let cases = [(datetime ! (2022 - 01 - 12 07 : 30 : 19.12345) , "2022-01-12T07:30:19.12345" ,) , (datetime ! (2022 - 01 - 12 07 : 30 : 19) , "2022-01-12T07:30:19.0") ,] ; for (value , expected) in cases { let value = value . to_value () ; if let Value :: String (s) = value { assert_eq ! (s , expected) ; } else { panic ! ("Unexpected Value type when formatting PrimitiveDateTime: {:?}" , value) ; } } } # [test] fn test_primitive_date_time_parse () { let cases = [("2022-01-12T07:30:19.12345" , datetime ! (2022 - 01 - 12 07 : 30 : 19.12345) ,) , ("2022-01-12T07:30:19.0" , datetime ! (2022 - 01 - 12 07 : 30 : 19)) ,] ; for (value , expected) in cases { let value = Value :: String (value . to_string ()) ; let parsed = < PrimitiveDateTime as ScalarType > :: parse (value) . unwrap () ; assert_eq ! (parsed , expected) ; } } }
};
}
