// Generated macro for verify_ne (macro)
macro_rules! Depcrate_assertionsverify_ne {
() => {
// Module: crate::assertions
// Provides: {"verify_ne"}
// Dependencies: {}
# [doc = " Checks whether the second argument is not equal to the first argument."] # [doc = ""] # [doc = " Evaluates to `Result::Ok(())` if they are not equal and"] # [doc = " `Result::Err(TestAssertionFailure)` if they are equal. The caller must then"] # [doc = " decide how to handle the `Err` variant. It has a few options:"] # [doc = "  * Abort the current function with the `?` operator. This requires that the"] # [doc = "    function return a suitable `Result`."] # [doc = "  * Log the test failure and continue by calling the method"] # [doc = "    `and_log_failure`."] # [doc = ""] # [doc = " Of course, one can also use all other standard methods on `Result`."] # [doc = ""] # [doc = " **Invoking this macro by itself does not cause a test failure to be recorded"] # [doc = " or output.** The resulting `Result` must be handled as described above to"] # [doc = " cause the test to be recorded as a failure."] # [doc = ""] # [doc = " Example:"] # [doc = " ```ignore"] # [doc = " use googletest::prelude::*;"] # [doc = ""] # [doc = " #[test]"] # [doc = " fn should_fail() -> Result<()> {"] # [doc = "     verify_ne!(1, 1)"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! verify_ne { ($ actual : expr , $ expected : expr $ (,) ?) => { $ crate :: verify_that ! (&$ actual , $ crate :: matchers :: not ($ crate :: matchers :: eq (&$ expected))) } ; }
};
}
