// Generated macro for macro_7116 (macro)
macro_rules! Depcrate_methodsmacro_7116 {
() => {
// Module: crate::methods
// Provides: {"macro_7116"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `option.map(_).unwrap_or(_)` or `option.map(_).unwrap_or_else(_)` or"] # [doc = " `result.map(_).unwrap_or_else(_)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, these can be written more concisely (resp.) as"] # [doc = " `option.map_or(_, _)`, `option.map_or_else(_, _)` and `result.map_or_else(_, _)`."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The order of the arguments is not in execution order"] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " # let option = Some(1);"] # [doc = " # let result: Result<usize, ()> = Ok(1);"] # [doc = " # fn some_function(foo: ()) -> usize { 1 }"] # [doc = " option.map(|a| a + 1).unwrap_or(0);"] # [doc = " option.map(|a| a > 10).unwrap_or(false);"] # [doc = " result.map(|a| a + 1).unwrap_or_else(some_function);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let option = Some(1);"] # [doc = " # let result: Result<usize, ()> = Ok(1);"] # [doc = " # fn some_function(foo: ()) -> usize { 1 }"] # [doc = " option.map_or(0, |a| a + 1);"] # [doc = " option.is_some_and(|a| a > 10);"] # [doc = " result.map_or_else(some_function, |a| a + 1);"] # [doc = " ```"] # [clippy :: version = "1.45.0"] pub MAP_UNWRAP_OR , pedantic , "using `.map(f).unwrap_or(a)` or `.map(f).unwrap_or_else(func)`, which are more succinctly expressed as `map_or(a, f)` or `map_or_else(a, f)`" }
};
}
