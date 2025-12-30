// Generated macro for expect_passes_rule (function)
macro_rules! Depcrate_validation_test_harnessexpect_passes_rule {
() => {
// Module: crate::validation::test_harness
// Provides: {"expect_passes_rule"}
// Dependencies: {}
pub (crate) fn expect_passes_rule < 'a , V , F , S > (factory : F , q : & 'a str) where S : ScalarValue + 'a , V : Visitor < 'a , S > + 'a , F : Fn () -> V , { expect_passes_rule_with_schema (QueryRoot , MutationRoot , SubscriptionRoot , factory , q) ; }
};
}
