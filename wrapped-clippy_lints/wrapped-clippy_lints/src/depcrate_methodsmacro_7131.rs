// Generated macro for macro_7131 (macro)
macro_rules! Depcrate_methodsmacro_7131 {
() => {
// Module: crate::methods
// Provides: {"macro_7131"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `.expect(&format!(...))`, `.expect(foo(..))`,"] # [doc = " etc., and suggests to use `unwrap_or_else` instead"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The function will always be called."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " If the function has side-effects, not calling it will"] # [doc = " change the semantics of the program, but you shouldn't rely on that anyway."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let foo = Some(String::new());"] # [doc = " # let err_code = \"418\";"] # [doc = " # let err_msg = \"I'm a teapot\";"] # [doc = " foo.expect(&format!(\"Err {}: {}\", err_code, err_msg));"] # [doc = ""] # [doc = " // or"] # [doc = ""] # [doc = " # let foo = Some(String::new());"] # [doc = " foo.expect(format!(\"Err {}: {}\", err_code, err_msg).as_str());"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let foo = Some(String::new());"] # [doc = " # let err_code = \"418\";"] # [doc = " # let err_msg = \"I'm a teapot\";"] # [doc = " foo.unwrap_or_else(|| panic!(\"Err {}: {}\", err_code, err_msg));"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub EXPECT_FUN_CALL , perf , "using any `expect` method with a function call" }
};
}
