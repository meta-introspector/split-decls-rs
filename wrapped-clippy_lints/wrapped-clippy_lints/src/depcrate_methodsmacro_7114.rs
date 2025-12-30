// Generated macro for macro_7114 (macro)
macro_rules! Depcrate_methodsmacro_7114 {
() => {
// Module: crate::methods
// Provides: {"macro_7114"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `.err().expect()` calls on the `Result` type."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `.expect_err()` can be called directly to avoid the extra type conversion from `err()`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```should_panic"] # [doc = " let x: Result<u32, &str> = Ok(10);"] # [doc = " x.err().expect(\"Testing err().expect()\");"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```should_panic"] # [doc = " let x: Result<u32, &str> = Ok(10);"] # [doc = " x.expect_err(\"Testing expect_err\");"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub ERR_EXPECT , style , r#"using `.err().expect("")` when `.expect_err("")` can be used"# }
};
}
