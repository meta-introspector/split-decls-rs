// Generated macro for macro_7034 (macro)
macro_rules! Depcrate_methodsmacro_7034 {
() => {
// Module: crate::methods
// Provides: {"macro_7034"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for unnecessary method chains that can be simplified into `if .. else ..`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This can be written more clearly with `if .. else ..`"] # [doc = ""] # [doc = " ### Limitations"] # [doc = " This lint currently only looks for usages of"] # [doc = " `.{then, then_some}(..).{unwrap_or, unwrap_or_else, unwrap_or_default}(..)`, but will be expanded"] # [doc = " to account for similar patterns."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = true;"] # [doc = " x.then_some(\"a\").unwrap_or(\"b\");"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = true;"] # [doc = " if x { \"a\" } else { \"b\" };"] # [doc = " ```"] # [clippy :: version = "1.64.0"] pub OBFUSCATED_IF_ELSE , style , "use of `.then_some(..).unwrap_or(..)` can be written \
    more clearly with `if .. else ..`" }
};
}
