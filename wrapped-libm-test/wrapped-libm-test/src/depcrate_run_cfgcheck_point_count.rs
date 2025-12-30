// Generated macro for check_point_count (function)
macro_rules! Depcrate_run_cfgcheck_point_count {
() => {
// Module: crate::run_cfg
// Provides: {"check_point_count"}
// Dependencies: {}
# [doc = " For domain tests, limit how many asymptotes or specified check points we test."] pub fn check_point_count (ctx : & CheckCtx) -> usize { assert_eq ! (ctx . gen_kind , GeneratorKind :: EdgeCases , "check_point_count is intended for edge case tests") ; let t_env = TestEnv :: from_env (ctx) ; if t_env . slow_platform || ! cfg ! (optimizations_enabled) { 4 } else { 10 } }
};
}
