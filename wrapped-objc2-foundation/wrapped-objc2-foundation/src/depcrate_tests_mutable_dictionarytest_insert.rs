// Generated macro for test_insert (function)
macro_rules! Depcrate_tests_mutable_dictionarytest_insert {
() => {
// Module: crate::tests::mutable_dictionary
// Provides: {"test_insert"}
// Dependencies: {}
# [test] fn test_insert () { let dict = < NSMutableDictionary < NSNumber , NSObject > > :: new () ; dict . insert (& * NSNumber :: new_i32 (1) , & NSObject :: new ()) ; dict . insert (& * NSNumber :: new_i32 (2) , & NSObject :: new ()) ; dict . insert (& * NSNumber :: new_i32 (3) , & NSObject :: new ()) ; dict . insert (& * NSNumber :: new_i32 (1) , & NSObject :: new ()) ; assert_eq ! (dict . len () , 3) ; }
};
}
