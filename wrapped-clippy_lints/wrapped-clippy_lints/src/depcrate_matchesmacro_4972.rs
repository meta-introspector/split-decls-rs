// Generated macro for macro_4972 (macro)
macro_rules! Depcrate_matchesmacro_4972 {
() => {
// Module: crate::matches
// Provides: {"macro_4972"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for arm which matches all errors with `Err(_)`"] # [doc = " and take drastic actions like `panic!`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is generally a bad practice, similar to"] # [doc = " catching all exceptions in java with `catch(Exception)`"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x: Result<i32, &str> = Ok(3);"] # [doc = " match x {"] # [doc = "     Ok(_) => println!(\"ok\"),"] # [doc = "     Err(_) => panic!(\"err\"),"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MATCH_WILD_ERR_ARM , pedantic , "a `match` with `Err(_)` arm and take drastic actions" }
};
}
