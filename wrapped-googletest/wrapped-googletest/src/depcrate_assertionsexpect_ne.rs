// Generated macro for expect_ne (macro)
macro_rules! Depcrate_assertionsexpect_ne {
() => {
// Module: crate::assertions
// Provides: {"expect_ne"}
// Dependencies: {}
# [doc = " Marks test as failed and continues execution if the second argument is"] # [doc = " equal to first argument."] # [doc = ""] # [doc = " This is a **not-fatal** failure. The test continues execution even after the"] # [doc = " macro execution."] # [doc = ""] # [doc = " This can only be invoked inside tests with the"] # [doc = " [`gtest`][crate::gtest] attribute. The failure must be generated"] # [doc = " in the same thread as that running the test itself."] # [doc = ""] # [doc = " Example:"] # [doc = " ```ignore"] # [doc = " use googletest::prelude::*;"] # [doc = ""] # [doc = " #[gtest]"] # [doc = " fn should_fail() {"] # [doc = "     expect_ne!(1, 1);"] # [doc = "     println!(\"This will print!\");"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " One may include formatted arguments in the failure message:"] # [doc = "```ignore"] # [doc = " use googletest::prelude::*;"] # [doc = ""] # [doc = " #[gtest]"] # [doc = " fn should_fail() {"] # [doc = "     let argument = \"argument\""] # [doc = "     expect_ne!(1, 1, \"custom failure message: {argument}\");"] # [doc = "     println!(\"This will print!\");"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! expect_ne { ($ actual : expr , $ expected : expr , $ ($ format_args : expr) ,+ $ (,) ?) => { { $ crate :: GoogleTestSupport :: and_log_failure_with_message ($ crate :: verify_ne ! ($ actual , $ expected) , || format ! ($ ($ format_args) ,*)) ; } } ; ($ actual : expr , $ expected : expr $ (,) ?) => { { $ crate :: GoogleTestSupport :: and_log_failure ($ crate :: verify_ne ! ($ actual , $ expected)) ; } } ; }
};
}
