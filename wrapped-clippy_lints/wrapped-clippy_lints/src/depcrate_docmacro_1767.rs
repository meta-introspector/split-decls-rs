// Generated macro for macro_1767 (macro)
macro_rules! Depcrate_docmacro_1767 {
() => {
// Module: crate::doc
// Provides: {"macro_1767"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `#[test]` in doctests unless they are marked with"] # [doc = " either `ignore`, `no_run` or `compile_fail`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Code in examples marked as `#[test]` will somewhat"] # [doc = " surprisingly not be run by `cargo test`. If you really want"] # [doc = " to show how to test stuff in an example, mark it `no_run` to"] # [doc = " make the intent clear."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " /// An example of a doctest with a `main()` function"] # [doc = " ///"] # [doc = " /// # Examples"] # [doc = " ///"] # [doc = " /// ```"] # [doc = " /// #[test]"] # [doc = " /// fn equality_works() {"] # [doc = " ///     assert_eq!(1_u8, 1);"] # [doc = " /// }"] # [doc = " /// ```"] # [doc = " fn test_attr_in_doctest() {"] # [doc = "     unimplemented!();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.76.0"] pub TEST_ATTR_IN_DOCTEST , suspicious , "presence of `#[test]` in code examples" }
};
}
