// Generated macro for test_i64_to_saturated_i32 (function)
macro_rules! Depcrate_helperstest_i64_to_saturated_i32 {
() => {
// Module: crate::helpers
// Provides: {"test_i64_to_saturated_i32"}
// Dependencies: {}
# [test] fn test_i64_to_saturated_i32 () { assert_eq ! (i64_to_saturated_i32 (i64 :: MIN) , i32 :: MIN) ; assert_eq ! (i64_to_saturated_i32 (- 2147483649) , - 2147483648) ; assert_eq ! (i64_to_saturated_i32 (- 2147483648) , - 2147483648) ; assert_eq ! (i64_to_saturated_i32 (- 2147483647) , - 2147483647) ; assert_eq ! (i64_to_saturated_i32 (- 2147483646) , - 2147483646) ; assert_eq ! (i64_to_saturated_i32 (- 100) , - 100) ; assert_eq ! (i64_to_saturated_i32 (0) , 0) ; assert_eq ! (i64_to_saturated_i32 (100) , 100) ; assert_eq ! (i64_to_saturated_i32 (2147483646) , 2147483646) ; assert_eq ! (i64_to_saturated_i32 (2147483647) , 2147483647) ; assert_eq ! (i64_to_saturated_i32 (2147483648) , 2147483647) ; assert_eq ! (i64_to_saturated_i32 (2147483649) , 2147483647) ; assert_eq ! (i64_to_saturated_i32 (i64 :: MAX) , i32 :: MAX) ; }
};
}
