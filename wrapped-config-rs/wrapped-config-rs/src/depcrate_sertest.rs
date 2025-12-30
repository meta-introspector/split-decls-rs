// Generated macro for test (module)
macro_rules! Depcrate_sertest {
() => {
// Module: crate::ser
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use serde :: { Deserialize , Serialize } ; use super :: * ; # [test] fn test_struct () { # [derive (Debug , Serialize , Deserialize , PartialEq , Eq)] struct Test { int : u32 , seq : Vec < String > , } let test = Test { int : 1 , seq : vec ! ["a" . to_owned () , "b" . to_owned ()] , } ; let config = Config :: try_from (& test) . unwrap () ; let actual : Test = config . try_deserialize () . unwrap () ; assert_eq ! (test , actual) ; } # [test] # [cfg (feature = "json")] fn test_nest () { let val = serde_json :: json ! { { "top" : { "num" : 1 , "array" : [2] , "nested" : [[3 , 4]] , "deep" : [{ "yes" : true , }] , "mixed" : [{ "boolish" : false , } , 42 , ["hi"] , { "inner" : 66 } , 23 ,] , } } } ; let config = Config :: try_from (& val) . unwrap () ; let output : serde_json :: Value = config . try_deserialize () . unwrap () ; assert_eq ! (val , output) ; } }
};
}
