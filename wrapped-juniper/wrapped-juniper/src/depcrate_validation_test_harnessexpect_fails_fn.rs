// Generated macro for expect_fails_fn (function)
macro_rules! Depcrate_validation_test_harnessexpect_fails_fn {
() => {
// Module: crate::validation::test_harness
// Provides: {"expect_fails_fn"}
// Dependencies: {}
pub (crate) fn expect_fails_fn < 'a , F , S > (visit_fn : F , q : & 'a str , expected_errors : & [RuleError]) where S : ScalarValue + 'a , F : FnOnce (& mut ValidatorContext < 'a , S > , & 'a Document < S >) , { expect_fails_fn_with_schema (QueryRoot , MutationRoot , visit_fn , q , expected_errors) ; }
};
}
