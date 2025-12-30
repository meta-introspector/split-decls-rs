// Generated macro for macro_9889 (macro)
macro_rules! Depcrate_trailing_empty_arraymacro_9889 {
() => {
// Module: crate::trailing_empty_array
// Provides: {"macro_9889"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Displays a warning when a struct with a trailing zero-sized array is declared without a `repr` attribute."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Zero-sized arrays aren't very useful in Rust itself, so such a struct is likely being created to pass to C code or in some other situation where control over memory layout matters (for example, in conjunction with manual allocation to make it easy to compute the offset of the array). Either way, `#[repr(C)]` (or another `repr` attribute) is needed."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct RarelyUseful {"] # [doc = "     some_field: u32,"] # [doc = "     last: [u32; 0],"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[repr(C)]"] # [doc = " struct MoreOftenUseful {"] # [doc = "     some_field: usize,"] # [doc = "     last: [u32; 0],"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.58.0"] pub TRAILING_EMPTY_ARRAY , nursery , "struct with a trailing zero-sized array but without `#[repr(C)]` or another `repr` attribute" }
};
}
