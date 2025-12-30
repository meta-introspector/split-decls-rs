// Generated macro for tests (module)
macro_rules! Depcrate_types_external_time_datetests {
() => {
// Module: crate::types::external::time_date
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use time :: { Date , macros :: date } ; use crate :: { ScalarType , Value } ; # [test] fn test_date_to_value () { let cases = [(date ! (1994 - 11 - 13) , "1994-11-13") , (date ! (2000 - 01 - 24) , "2000-01-24") ,] ; for (value , expected) in cases { let value = value . to_value () ; if let Value :: String (s) = value { assert_eq ! (s , expected) ; } else { panic ! ("Unexpected Value type when formatting PrimitiveDateTime: {:?}" , value) ; } } } # [test] fn test_date_parse () { let cases = [("1994-11-13" , date ! (1994 - 11 - 13)) , ("2000-01-24" , date ! (2000 - 01 - 24)) ,] ; for (value , expected) in cases { let value = Value :: String (value . to_string ()) ; let parsed = < Date as ScalarType > :: parse (value) . unwrap () ; assert_eq ! (parsed , expected) ; } } }
};
}
