// Generated macro for macro_7129 (macro)
macro_rules! Depcrate_methodsmacro_7129 {
() => {
// Module: crate::methods
// Provides: {"macro_7129"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `.or(foo(..))`, `.unwrap_or(foo(..))`,"] # [doc = " `.or_insert(foo(..))` etc., and suggests to use `.or_else(|| foo(..))`,"] # [doc = " `.unwrap_or_else(|| foo(..))`, `.unwrap_or_default()` or `.or_default()`"] # [doc = " etc. instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The function will always be called. This is only bad if it allocates or"] # [doc = " does some non-trivial amount of work."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " If the function has side-effects, not calling it will change the"] # [doc = " semantic of the program, but you shouldn't rely on that."] # [doc = ""] # [doc = " The lint also cannot figure out whether the function you call is"] # [doc = " actually expensive to call or not."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let foo = Some(String::new());"] # [doc = " foo.unwrap_or(String::from(\"empty\"));"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let foo = Some(String::new());"] # [doc = " foo.unwrap_or_else(|| String::from(\"empty\"));"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub OR_FUN_CALL , nursery , "using any `*or` method with a function call, which suggests `*or_else`" }
};
}
