// Generated macro for macro_3147 (macro)
macro_rules! Depcrate_items_after_test_modulemacro_3147 {
() => {
// Module: crate::items_after_test_module
// Provides: {"macro_3147"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Triggers if an item is declared after the testing module marked with `#[cfg(test)]`."] # [doc = " ### Why is this bad?"] # [doc = " Having items declared after the testing module is confusing and may lead to bad test coverage."] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[cfg(test)]"] # [doc = " mod tests {"] # [doc = "     // [...]"] # [doc = " }"] # [doc = ""] # [doc = " fn my_function() {"] # [doc = "     // [...]"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn my_function() {"] # [doc = "     // [...]"] # [doc = " }"] # [doc = ""] # [doc = " #[cfg(test)]"] # [doc = " mod tests {"] # [doc = "     // [...]"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.71.0"] pub ITEMS_AFTER_TEST_MODULE , style , "An item was found after the testing module `tests`" }
};
}
