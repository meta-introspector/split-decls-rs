// Generated macro for skip_sys_checks (function)
macro_rules! Depcrate_benchskip_sys_checks {
() => {
// Module: crate::bench
// Provides: {"skip_sys_checks"}
// Dependencies: {}
# [doc = " Still run benchmarks/tests but don't check correctness between compiler-builtins and"] # [doc = " builtin system functions functions"] pub fn skip_sys_checks (test_name : & str) -> bool { const ALWAYS_SKIPPED : & [& str] = & ["extend_f16_f32" , "trunc_f32_f16" , "trunc_f64_f16" ,] ; const X86_NO_SSE_SKIPPED : & [& str] = & ["add_f128" , "sub_f128" , "mul_f128" , "div_f128" , "powi_f32" , "powi_f64" ,] ; const WINDOWS_SKIPPED : & [& str] = & ["conv_f32_u128" , "conv_f32_i128" , "conv_f64_u128" , "conv_f64_i128" ,] ; if cfg ! (target_arch = "arm") { return true ; } if ALWAYS_SKIPPED . contains (& test_name) { return true ; } if cfg ! (x86_no_sse) && X86_NO_SSE_SKIPPED . contains (& test_name) { return true ; } if cfg ! (target_family = "windows") && WINDOWS_SKIPPED . contains (& test_name) { return true ; } false }
};
}
