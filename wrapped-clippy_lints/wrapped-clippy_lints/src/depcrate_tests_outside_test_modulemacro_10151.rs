// Generated macro for macro_10151 (macro)
macro_rules! Depcrate_tests_outside_test_modulemacro_10151 {
() => {
// Module: crate::tests_outside_test_module
// Provides: {"macro_10151"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Triggers when a testing function (marked with the `#[test]` attribute) isn't inside a testing module"] # [doc = " (marked with `#[cfg(test)]`)."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " The idiomatic (and more performant) way of writing tests is inside a testing module (flagged with `#[cfg(test)]`),"] # [doc = " having test functions outside of this module is confusing and may lead to them being \"hidden\"."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[test]"] # [doc = " fn my_cool_test() {"] # [doc = "     // [...]"] # [doc = " }"] # [doc = ""] # [doc = " #[cfg(test)]"] # [doc = " mod tests {"] # [doc = "     // [...]"] # [doc = " }"] # [doc = ""] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[cfg(test)]"] # [doc = " mod tests {"] # [doc = "     #[test]"] # [doc = "     fn my_cool_test() {"] # [doc = "         // [...]"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.70.0"] pub TESTS_OUTSIDE_TEST_MODULE , restriction , "A test function is outside the testing module." }
};
}
