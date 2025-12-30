// Generated macro for expect_fails_rule (function)
macro_rules! Depcrate_validation_test_harnessexpect_fails_rule {
() => {
// Module: crate::validation::test_harness
// Provides: {"expect_fails_rule"}
// Dependencies: {}
pub (crate) fn expect_fails_rule < 'a , V , F , S > (factory : F , q : & 'a str , expected_errors : & [RuleError]) where S : ScalarValue + 'a , V : Visitor < 'a , S > + 'a , F : Fn () -> V , { expect_fails_rule_with_schema (QueryRoot , MutationRoot , factory , q , expected_errors) ; }
};
}
