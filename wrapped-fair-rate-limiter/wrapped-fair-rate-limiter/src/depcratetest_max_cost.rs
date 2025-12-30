// Generated macro for test_max_cost (function)
macro_rules! Depcratetest_max_cost {
() => {
// Module: crate
// Provides: {"test_max_cost"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_max_cost () { assert_eq ! (100 , max_cost (100 , 0 , 0)) ; assert_eq ! (100 , max_cost (100 , 0 , 1)) ; assert_eq ! (100 , max_cost (100 , 1 , 1)) ; assert_eq ! (100 , max_cost (100 , 100 , 1)) ; assert_eq ! (100 , max_cost (100 , 0 , 2)) ; assert_eq ! (100 , max_cost (100 , 75 , 2)) ; assert_eq ! (98 , max_cost (100 , 76 , 2)) ; assert_eq ! (70 , max_cost (100 , 90 , 2)) ; assert_eq ! (52 , max_cost (100 , 99 , 2)) ; assert_eq ! (50 , max_cost (100 , 100 , 2)) ; assert_eq ! (50 , max_cost (100 , 150 , 2)) ; assert_eq ! (1000 , max_cost (1000 , 0 , 10)) ; assert_eq ! (1000 , max_cost (1000 , 750 , 10)) ; assert_eq ! (996 , max_cost (1000 , 751 , 10)) ; assert_eq ! (459 , max_cost (1000 , 900 , 10)) ; assert_eq ! (103 , max_cost (1000 , 999 , 10)) ; assert_eq ! (99 , max_cost (1000 , 1000 , 10)) ; assert_eq ! (100 , max_cost (1000 , 1500 , 10)) ; }
};
}
