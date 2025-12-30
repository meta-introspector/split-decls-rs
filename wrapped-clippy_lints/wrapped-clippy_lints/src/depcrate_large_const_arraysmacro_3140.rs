// Generated macro for macro_3140 (macro)
macro_rules! Depcrate_large_const_arraysmacro_3140 {
() => {
// Module: crate::large_const_arrays
// Provides: {"macro_3140"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for large `const` arrays that should"] # [doc = " be defined as `static` instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Performance: const variables are inlined upon use."] # [doc = " Static items result in only one instance and has a fixed location in memory."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " pub const a = [0u32; 1_000_000];"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " pub static a = [0u32; 1_000_000];"] # [doc = " ```"] # [clippy :: version = "1.44.0"] pub LARGE_CONST_ARRAYS , perf , "large non-scalar const array may cause performance overhead" }
};
}
