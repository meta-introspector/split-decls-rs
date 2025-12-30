// Generated macro for check_near_count (function)
macro_rules! Depcrate_run_cfgcheck_near_count {
() => {
// Module: crate::run_cfg
// Provides: {"check_near_count"}
// Dependencies: {}
# [doc = " When validating points of interest (e.g. asymptotes, inflection points, extremes), also check"] # [doc = " this many surrounding values."] pub fn check_near_count (ctx : & CheckCtx) -> u64 { assert_eq ! (ctx . gen_kind , GeneratorKind :: EdgeCases , "check_near_count is intended for edge case tests") ; if cfg ! (optimizations_enabled) { match ctx . input_count () { 1 | 2 => 100 , 3 => 50 , x => panic ! ("unexpected argument count {x}") , } } else { 8 } }
};
}
