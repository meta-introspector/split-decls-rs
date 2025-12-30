// Generated macro for step_cycle_debug (function)
macro_rules! Depcrate_core_builder_testsstep_cycle_debug {
() => {
// Module: crate::core::builder::tests
// Provides: {"step_cycle_debug"}
// Dependencies: {}
# [doc = " When bootstrap detects a step dependency cycle (which is a bug), its panic"] # [doc = " message should show the actual steps on the stack, not just several copies"] # [doc = " of `Any { .. }`."] # [test] fn step_cycle_debug () { let config = configure_with_args (& ["run" , "cyclic-step"] , & [TEST_TRIPLE_1] , & [TEST_TRIPLE_1]) ; let err = panic :: catch_unwind (| | run_build (& config . paths . clone () , config)) . unwrap_err () ; let err = err . downcast_ref :: < String > () . unwrap () . as_str () ; assert ! (! err . contains ("Any")) ; assert ! (err . contains ("CyclicStep { n: 1 }")) ; }
};
}
