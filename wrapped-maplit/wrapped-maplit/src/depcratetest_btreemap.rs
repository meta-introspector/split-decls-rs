// Generated macro for test_btreemap (function)
macro_rules! Depcratetest_btreemap {
() => {
// Module: crate
// Provides: {"test_btreemap"}
// Dependencies: {}
# [test] fn test_btreemap () { use std :: collections :: BTreeMap ; let names = btreemap ! { 1 => "one" , 2 => "two" , } ; assert_eq ! (names . len () , 2) ; assert_eq ! (names [& 1] , "one") ; assert_eq ! (names [& 2] , "two") ; assert_eq ! (names . get (& 3) , None) ; let empty : BTreeMap < i32 , i32 > = btreemap ! { } ; assert_eq ! (empty . len () , 0) ; let _nested_compiles = btreemap ! { 1 => btreemap ! { 0 => 1 + 2 , } , 2 => btreemap ! { 1 => 1 , } , } ; }
};
}
