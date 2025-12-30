// Generated macro for test_filters (function)
macro_rules! Depcrate_filterstest_filters {
() => {
// Module: crate::filters
// Provides: {"test_filters"}
// Dependencies: {}
# [test] fn test_filters () { let mut filters = Filters :: default () ; filters . add ("\\bhello\\b" , "[NAME]") ; filters . add ("(a)" , "[$1]") ; assert_eq ! (filters . apply_to ("hellohello hello abc") , "hellohello [NAME] [a]bc") ; }
};
}
