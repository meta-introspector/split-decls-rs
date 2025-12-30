// Generated macro for macro_6970 (macro)
macro_rules! Depcrate_methodsmacro_6970 {
() => {
// Module: crate::methods
// Provides: {"macro_6970"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.map_or(None, _)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely as"] # [doc = " `_.and_then(_)`."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The order of the arguments is not in execution order."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let opt = Some(1);"] # [doc = " opt.map_or(None, |a| Some(a + 1));"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let opt = Some(1);"] # [doc = " opt.and_then(|a| Some(a + 1));"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub OPTION_MAP_OR_NONE , style , "using `Option.map_or(None, f)`, which is more succinctly expressed as `and_then(f)`" }
};
}
