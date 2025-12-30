// Generated macro for macro_3207 (macro)
macro_rules! Depcrate_large_stack_arraysmacro_3207 {
() => {
// Module: crate::large_stack_arrays
// Provides: {"macro_3207"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for local arrays that may be too large."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Large local arrays may cause stack overflow."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let a = [0u32; 1_000_000];"] # [doc = " ```"] # [clippy :: version = "1.41.0"] pub LARGE_STACK_ARRAYS , pedantic , "allocating large arrays on stack may cause stack overflow" }
};
}
