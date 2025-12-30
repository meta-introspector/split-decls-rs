// Generated macro for macro_9416 (macro)
macro_rules! Depcrate_redundant_test_prefixmacro_9416 {
() => {
// Module: crate::redundant_test_prefix
// Provides: {"macro_9416"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for test functions (functions annotated with `#[test]`) that are prefixed"] # [doc = " with `test_` which is redundant."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is redundant because test functions are already annotated with `#[test]`."] # [doc = " Moreover, it clutters the output of `cargo test` since test functions are expanded as"] # [doc = " `module::tests::test_use_case` in the output. Without the redundant prefix, the output"] # [doc = " becomes `module::tests::use_case`, which is more readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[cfg(test)]"] # [doc = " mod tests {"] # [doc = "   use super::*;"] # [doc = ""] # [doc = "   #[test]"] # [doc = "   fn test_use_case() {"] # [doc = "       // test code"] # [doc = "   }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[cfg(test)]"] # [doc = " mod tests {"] # [doc = "   use super::*;"] # [doc = ""] # [doc = "   #[test]"] # [doc = "   fn use_case() {"] # [doc = "       // test code"] # [doc = "   }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.88.0"] pub REDUNDANT_TEST_PREFIX , restriction , "redundant `test_` prefix in test function name" }
};
}
