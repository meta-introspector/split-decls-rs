// Generated macro for macro_7163 (macro)
macro_rules! Depcrate_methodsmacro_7163 {
() => {
// Module: crate::methods
// Provides: {"macro_7163"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.map(_).collect::<Result<(), _>()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `try_for_each` instead is more readable and idiomatic."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " (0..3).map(|t| Err(t)).collect::<Result<(), _>>();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " (0..3).try_for_each(|t| Err(t));"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub MAP_COLLECT_RESULT_UNIT , style , "using `.map(_).collect::<Result<(),_>()`, which can be replaced with `try_for_each`" }
};
}
