// Generated macro for macro_2873 (macro)
macro_rules! Depcrate_indexing_slicingmacro_2873 {
() => {
// Module: crate::indexing_slicing
// Provides: {"macro_2873"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for out of bounds array indexing with a constant"] # [doc = " index."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This will always panic at runtime."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,no_run"] # [doc = " let x = [1, 2, 3, 4];"] # [doc = ""] # [doc = " x[9];"] # [doc = " &x[2..9];"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let x = [1, 2, 3, 4];"] # [doc = " // Index within bounds"] # [doc = ""] # [doc = " x[0];"] # [doc = " x[3];"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub OUT_OF_BOUNDS_INDEXING , correctness , "out of bounds constant indexing" }
};
}
