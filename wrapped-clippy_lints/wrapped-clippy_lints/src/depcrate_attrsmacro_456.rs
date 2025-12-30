// Generated macro for macro_456 (macro)
macro_rules! Depcrate_attrsmacro_456 {
() => {
// Module: crate::attrs
// Provides: {"macro_456"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `#[should_panic]` attributes without specifying the expected panic message."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The expected panic message should be specified to ensure that the test is actually"] # [doc = " panicking with the expected message, and not another unrelated panic."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn random() -> i32 { 0 }"] # [doc = ""] # [doc = " #[should_panic]"] # [doc = " #[test]"] # [doc = " fn my_test() {"] # [doc = "     let _ = 1 / random();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn random() -> i32 { 0 }"] # [doc = ""] # [doc = " #[should_panic = \"attempt to divide by zero\"]"] # [doc = " #[test]"] # [doc = " fn my_test() {"] # [doc = "     let _ = 1 / random();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.74.0"] pub SHOULD_PANIC_WITHOUT_EXPECT , pedantic , "ensures that all `should_panic` attributes specify its expected panic message" }
};
}
