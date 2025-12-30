// Generated macro for macro_4037 (macro)
macro_rules! Depcrate_manual_bitsmacro_4037 {
() => {
// Module: crate::manual_bits
// Provides: {"macro_4037"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `size_of::<T>() * 8` when"] # [doc = " `T::BITS` is available."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Can be written as the shorter `T::BITS`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " size_of::<usize>() * 8;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " usize::BITS as usize;"] # [doc = " ```"] # [clippy :: version = "1.60.0"] pub MANUAL_BITS , style , "manual implementation of `size_of::<T>() * 8` can be simplified with `T::BITS`" }
};
}
