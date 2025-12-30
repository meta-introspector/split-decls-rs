// Generated macro for macro_7255 (macro)
macro_rules! Depcrate_misc_earlymacro_7255 {
() => {
// Module: crate::misc_early
// Provides: {"macro_7255"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for patterns in the form `name @ _`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's almost always more readable to just use direct"] # [doc = " bindings."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let v = Some(\"abc\");"] # [doc = " match v {"] # [doc = "     Some(x) => (),"] # [doc = "     y @ _ => (),"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let v = Some(\"abc\");"] # [doc = " match v {"] # [doc = "     Some(x) => (),"] # [doc = "     y => (),"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub REDUNDANT_PATTERN , style , "using `name @ _` in a pattern" }
};
}
