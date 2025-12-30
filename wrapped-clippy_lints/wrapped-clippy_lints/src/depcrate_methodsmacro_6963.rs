// Generated macro for macro_6963 (macro)
macro_rules! Depcrate_methodsmacro_6963 {
() => {
// Module: crate::methods
// Provides: {"macro_6963"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `.expect()` or `.expect_err()` calls on `Result`s and `.expect()` call on `Option`s."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Usually it is better to handle the `None` or `Err` case."] # [doc = " Still, for a lot of quick-and-dirty code, `expect` is a good choice, which is why"] # [doc = " this lint is `Allow` by default."] # [doc = ""] # [doc = " `result.expect()` will let the thread panic on `Err`"] # [doc = " values. Normally, you want to implement more sophisticated error handling,"] # [doc = " and propagate errors upwards with `?` operator."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```rust,ignore"] # [doc = " # let option = Some(1);"] # [doc = " # let result: Result<usize, ()> = Ok(1);"] # [doc = " option.expect(\"one\");"] # [doc = " result.expect(\"one\");"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " # let option = Some(1);"] # [doc = " # let result: Result<usize, ()> = Ok(1);"] # [doc = " option?;"] # [doc = ""] # [doc = " // or"] # [doc = ""] # [doc = " result?;"] # [doc = " ```"] # [clippy :: version = "1.45.0"] pub EXPECT_USED , restriction , "using `.expect()` on `Result` or `Option`, which might be better handled" }
};
}
