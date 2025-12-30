// Generated macro for test_is_builder_target (function)
macro_rules! Depcrate_core_builder_teststest_is_builder_target {
() => {
// Module: crate::core::builder::tests
// Provides: {"test_is_builder_target"}
// Dependencies: {}
# [test] fn test_is_builder_target () { let target1 = TargetSelection :: from_user (TEST_TRIPLE_1) ; let target2 = TargetSelection :: from_user (TEST_TRIPLE_2) ; for (target1 , target2) in [(target1 , target2) , (target2 , target1)] { let mut config = configure ("build" , & [] , & []) ; config . host_target = target1 ; let build = Build :: new (config) ; let builder = Builder :: new (& build) ; assert ! (builder . config . is_host_target (target1)) ; assert ! (! builder . config . is_host_target (target2)) ; } }
};
}
