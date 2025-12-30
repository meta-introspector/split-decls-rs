// Generated macro for test_hashmap (function)
macro_rules! Depcratetest_hashmap {
() => {
// Module: crate
// Provides: {"test_hashmap"}
// Dependencies: {}
# [test] fn test_hashmap () { use std :: collections :: HashMap ; use std :: collections :: HashSet ; let names = hashmap ! { 1 => "one" , 2 => "two" , } ; assert_eq ! (names . len () , 2) ; assert_eq ! (names [& 1] , "one") ; assert_eq ! (names [& 2] , "two") ; assert_eq ! (names . get (& 3) , None) ; let empty : HashMap < i32 , i32 > = hashmap ! { } ; assert_eq ! (empty . len () , 0) ; let _nested_compiles = hashmap ! { 1 => hashmap ! { 0 => 1 + 2 , } , 2 => hashmap ! { 1 => 1 , } , } ; let _ : HashMap < String , i32 > = convert_args ! (keys = String :: from , hashmap ! ("one" => 1 , "two" => 2 ,)) ; let _ : HashMap < String , i32 > = convert_args ! (keys = String :: from , values = __id , hashmap ! ("one" => 1 , "two" => 2 ,)) ; let names : HashSet < String > = convert_args ! (hashset ! ("one" , "two" ,)) ; assert ! (names . contains ("one")) ; assert ! (names . contains ("two")) ; let lengths : HashSet < usize > = convert_args ! (keys = str :: len , hashset ! ("one" , "two" ,)) ; assert_eq ! (lengths . len () , 1) ; let _no_trailing : HashSet < usize > = convert_args ! (keys = str :: len , hashset ! ("one" , "two")) ; }
};
}
