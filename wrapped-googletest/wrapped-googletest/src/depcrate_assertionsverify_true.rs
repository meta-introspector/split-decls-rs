// Generated macro for verify_true (macro)
macro_rules! Depcrate_assertionsverify_true {
() => {
// Module: crate::assertions
// Provides: {"verify_true"}
// Dependencies: {}
# [doc = " Verify if the condition evaluates to true and returns `Result`."] # [doc = ""] # [doc = " Evaluates to `Result::Ok(())` if the condition is true and"] # [doc = " `Result::Err(TestAssertionFailure)` if it evaluates to false. The caller"] # [doc = " must then decide how to handle the `Err` variant. It has a few options:"] # [doc = "   * Abort the current function with the `?` operator. This requires that the"] # [doc = "     function return a suitable `Result`."] # [doc = "   * Log the failure and continue by calling the method `and_log_failure`."] # [doc = ""] # [doc = " Of course, one can also use all other standard methods on `Result`."] # [doc = ""] # [doc = " **Invoking this macro by itself does not cause a test failure to be recorded"] # [doc = " or output.** The resulting `Result` must be handled as described above to"] # [doc = " cause the test to be recorded as a failure."] # [doc = ""] # [doc = " Example:"] # [doc = " ```ignore"] # [doc = " use googletest::prelude::*;"] # [doc = ""] # [doc = " #[test]"] # [doc = " fn should_fail() -> Result<()> {"] # [doc = "     verify_true!(2 + 2 == 5)"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! verify_true { ($ condition : expr) => { { use $ crate :: assertions :: internal :: Subject as _ ; ($ condition) . check ($ crate :: matchers :: eq (true) , stringify ! ($ condition)) } } ; }
};
}
