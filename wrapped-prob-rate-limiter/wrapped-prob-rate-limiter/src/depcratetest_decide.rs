// Generated macro for test_decide (function)
macro_rules! Depcratetest_decide {
() => {
// Module: crate
// Provides: {"test_decide"}
// Dependencies: {}
# [cfg (test)] # [test] # [allow (clippy :: unreadable_literal)] fn test_decide () { assert ! (! decide (0 , 0 , || unreachable ! ())) ; assert ! (decide (0 , 100 , || unreachable ! ())) ; assert ! (decide (50 , 100 , || unreachable ! ())) ; assert ! (decide (75 , 100 , || unreachable ! ())) ; assert ! (decide (76 , 100 , || 0.999999)) ; assert ! (! decide (76 , 100 , || 0.0)) ; assert ! (! decide (85 , 100 , || 0.15)) ; assert ! (decide (85 , 100 , || 0.17)) ; assert ! (! decide (90 , 100 , || 0.35)) ; assert ! (decide (90 , 100 , || 0.37)) ; assert ! (! decide (95 , 100 , || 0.63)) ; assert ! (decide (95 , 100 , || 0.65)) ; assert ! (! decide (99 , 100 , || 0.92)) ; assert ! (decide (99 , 100 , || 0.93)) ; assert ! (! decide (100 , 100 , || unreachable ! ())) ; assert ! (! decide (101 , 100 , || unreachable ! ())) ; }
};
}
