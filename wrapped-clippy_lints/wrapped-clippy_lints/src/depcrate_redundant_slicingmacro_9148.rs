// Generated macro for macro_9148 (macro)
macro_rules! Depcrate_redundant_slicingmacro_9148 {
() => {
// Module: crate::redundant_slicing
// Provides: {"macro_9148"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for slicing expressions which are equivalent to dereferencing the"] # [doc = " value."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Some people may prefer to dereference rather than slice."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let vec = vec![1, 2, 3];"] # [doc = " let slice = &vec[..];"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let vec = vec![1, 2, 3];"] # [doc = " let slice = &*vec;"] # [doc = " ```"] # [clippy :: version = "1.61.0"] pub DEREF_BY_SLICING , restriction , "slicing instead of dereferencing" }
};
}
