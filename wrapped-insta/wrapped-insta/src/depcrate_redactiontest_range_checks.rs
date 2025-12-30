// Generated macro for test_range_checks (function)
macro_rules! Depcrate_redactiontest_range_checks {
() => {
// Module: crate::redaction
// Provides: {"test_range_checks"}
// Dependencies: {}
# [test] fn test_range_checks () { use similar_asserts :: assert_eq ; assert_eq ! (PathItem :: Index (0 , 10) . range_check (None , Some (- 1)) , true) ; assert_eq ! (PathItem :: Index (9 , 10) . range_check (None , Some (- 1)) , false) ; assert_eq ! (PathItem :: Index (0 , 10) . range_check (Some (1) , Some (- 1)) , false) ; assert_eq ! (PathItem :: Index (1 , 10) . range_check (Some (1) , Some (- 1)) , true) ; assert_eq ! (PathItem :: Index (9 , 10) . range_check (Some (1) , Some (- 1)) , false) ; assert_eq ! (PathItem :: Index (0 , 10) . range_check (Some (1) , None) , false) ; assert_eq ! (PathItem :: Index (1 , 10) . range_check (Some (1) , None) , true) ; assert_eq ! (PathItem :: Index (9 , 10) . range_check (Some (1) , None) , true) ; }
};
}
