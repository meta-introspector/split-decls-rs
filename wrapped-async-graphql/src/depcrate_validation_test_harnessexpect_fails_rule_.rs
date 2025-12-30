// Generated macro for expect_fails_rule_ (function)
macro_rules! Depcrate_validation_test_harnessexpect_fails_rule_ {
() => {
// Module: crate::validation::test_harness
// Provides: {"expect_fails_rule_"}
// Dependencies: {}
pub (crate) fn expect_fails_rule_ < 'a , V , F > (doc : & 'a ExecutableDocument , factory : F) where V : Visitor < 'a > + 'a , F : Fn () -> V , { if validate (doc , factory) . is_ok () { panic ! ("Expected rule to fail, but no errors were found") ; } }
};
}
