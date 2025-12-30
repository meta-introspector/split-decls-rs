// Generated macro for expect_gt (macro)
macro_rules! Depcrate_assertionsexpect_gt {
() => {
// Module: crate::assertions
// Provides: {"expect_gt"}
// Dependencies: {}
# [doc = " Marks test as failed and continues execution if the first argument is"] # [doc = " not greater than the second argument."] # [doc = ""] # [doc = " This is a **not-fatal** failure. The test continues execution even after the"] # [doc = " macro execution."] # [doc = ""] # [doc = " This can only be invoked inside tests with the"] # [doc = " [`gtest`][crate::gtest] attribute. The failure must be generated"] # [doc = " in the same thread as that running the test itself."] # [doc = ""] # [doc = " Example:"] # [doc = " ```ignore"] # [doc = " use googletest::prelude::*;"] # [doc = ""] # [doc = " #[gtest]"] # [doc = " fn should_fail() {"] # [doc = "     expect_gt!(1, 2);"] # [doc = "     println!(\"This will print!\");"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " One may include formatted arguments in the failure message:"] # [doc = "```ignore"] # [doc = " use googletest::prelude::*;"] # [doc = ""] # [doc = " #[gtest]"] # [doc = " fn should_fail() {"] # [doc = "     let argument = \"argument\""] # [doc = "     expect_gt!(1, 2, \"custom failure message: {argument}\");"] # [doc = "     println!(\"This will print!\");"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! expect_gt { ($ actual : expr , $ expected : expr , $ ($ format_args : expr) ,+ $ (,) ?) => { { $ crate :: GoogleTestSupport :: and_log_failure_with_message ($ crate :: verify_gt ! ($ actual , $ expected) , || format ! ($ ($ format_args) ,*)) ; } } ; ($ actual : expr , $ expected : expr $ (,) ?) => { { $ crate :: GoogleTestSupport :: and_log_failure ($ crate :: verify_gt ! ($ actual , $ expected)) ; } } ; }
};
}
