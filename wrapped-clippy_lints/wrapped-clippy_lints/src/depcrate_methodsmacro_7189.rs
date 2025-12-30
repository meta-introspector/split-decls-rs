// Generated macro for macro_7189 (macro)
macro_rules! Depcrate_methodsmacro_7189 {
() => {
// Module: crate::methods
// Provides: {"macro_7189"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Finds patterns that reimplement `Option::ok_or`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Concise code helps focusing on behavior instead of boilerplate."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " let foo: Option<i32> = None;"] # [doc = " foo.map_or(Err(\"error\"), |v| Ok(v));"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let foo: Option<i32> = None;"] # [doc = " foo.ok_or(\"error\");"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub MANUAL_OK_OR , style , "finds patterns that can be encoded more concisely with `Option::ok_or`" }
};
}
