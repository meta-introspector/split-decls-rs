// Generated macro for verify_le (macro)
macro_rules! Depcrate_assertionsverify_le {
() => {
// Module: crate::assertions
// Provides: {"verify_le"}
// Dependencies: {}
# [doc = " Checks whether the first argument is less than or equal to the second"] # [doc = " argument."] # [doc = ""] # [doc = " Evaluates to `Result::Ok(())` if the first argument is less than or equal to"] # [doc = " the second and `Result::Err(TestAssertionFailure)` if it is greater. The"] # [doc = " caller must then decide how to handle the `Err` variant. It has a few"] # [doc = " options:"] # [doc = "  * Abort the current function with the `?` operator. This requires that the"] # [doc = "    function return a suitable `Result`."] # [doc = "  * Log the test failure and continue by calling the method"] # [doc = "    `and_log_failure`."] # [doc = ""] # [doc = " Of course, one can also use all other standard methods on `Result`."] # [doc = ""] # [doc = " **Invoking this macro by itself does not cause a test failure to be recorded"] # [doc = " or output.** The resulting `Result` must be handled as described above to"] # [doc = " cause the test to be recorded as a failure."] # [doc = ""] # [doc = " Example:"] # [doc = " ```ignore"] # [doc = " use googletest::prelude::*;"] # [doc = ""] # [doc = " #[test]"] # [doc = " fn should_fail() -> Result<()> {"] # [doc = "     verify_le!(2, 1)"] # [doc = " }"] # [macro_export] macro_rules ! verify_le { ($ actual : expr , $ expected : expr $ (,) ?) => { $ crate :: verify_that ! ($ actual , $ crate :: matchers :: le ($ expected)) } ; }
};
}
