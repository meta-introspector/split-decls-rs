// Generated macro for macro_6966 (macro)
macro_rules! Depcrate_methodsmacro_6966 {
() => {
// Module: crate::methods
// Provides: {"macro_6966"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `ok().expect(..)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Because you usually call `expect()` on the `Result`"] # [doc = " directly to get a better error message."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = Ok::<_, ()>(());"] # [doc = " x.ok().expect(\"why did I do this again?\");"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let x = Ok::<_, ()>(());"] # [doc = " x.expect(\"why did I do this again?\");"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub OK_EXPECT , style , "using `ok().expect()`, which gives worse error messages than calling `expect` directly on the Result" }
};
}
