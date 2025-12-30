// Generated macro for macro_9562 (macro)
macro_rules! Depcrate_reserve_after_initializationmacro_9562 {
() => {
// Module: crate::reserve_after_initialization
// Provides: {"macro_9562"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Informs the user about a more concise way to create a vector with a known capacity."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `Vec::with_capacity` constructor is less complex."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut v: Vec<usize> = vec![];"] # [doc = " v.reserve(10);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut v: Vec<usize> = Vec::with_capacity(10);"] # [doc = " ```"] # [clippy :: version = "1.74.0"] pub RESERVE_AFTER_INITIALIZATION , complexity , "`reserve` called immediately after `Vec` creation" }
};
}
