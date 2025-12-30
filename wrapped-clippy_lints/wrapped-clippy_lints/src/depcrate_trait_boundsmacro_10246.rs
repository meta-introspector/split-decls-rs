// Generated macro for macro_10246 (macro)
macro_rules! Depcrate_trait_boundsmacro_10246 {
() => {
// Module: crate::trait_bounds
// Provides: {"macro_10246"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint warns about unnecessary type repetitions in trait bounds"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Repeating the type for every bound makes the code"] # [doc = " less readable than combining the bounds"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " pub fn foo<T>(t: T) where T: Copy, T: Clone {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " pub fn foo<T>(t: T) where T: Copy + Clone {}"] # [doc = " ```"] # [clippy :: version = "1.38.0"] pub TYPE_REPETITION_IN_BOUNDS , nursery , "types are repeated unnecessarily in trait bounds, use `+` instead of using `T: _, T: _`" }
};
}
