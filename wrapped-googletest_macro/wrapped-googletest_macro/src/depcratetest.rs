// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [doc = " Alias for [`googletest::gtest`]."] # [doc = ""] # [doc = " Generally, prefer using `#[gtest]` to mark googletest-based tests."] # [doc = ""] # [doc = " Use `#[test]` instead of `#[gtest]` to satisfy compatibility"] # [doc = " requirements. For example, the rstest crate can be composed with other test"] # [doc = " attributes but it requires the attribute to be named `test`."] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[rstest]"] # [doc = " #[googletest::test]"] # [doc = " fn rstest_with_googletest() -> Result<()> {"] # [doc = "   verify_that!(1, eq(1))"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`googletest::gtest`]: attr.gtest.html"] # [proc_macro_attribute] pub fn test (args : proc_macro :: TokenStream , input : proc_macro :: TokenStream ,) -> proc_macro :: TokenStream { gtest (args , input) }
};
}
