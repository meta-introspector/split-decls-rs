// Generated macro for expect_passes_rule_ (function)
macro_rules! Depcrate_validation_test_harnessexpect_passes_rule_ {
() => {
// Module: crate::validation::test_harness
// Provides: {"expect_passes_rule_"}
// Dependencies: {}
pub (crate) fn expect_passes_rule_ < 'a , V , F > (doc : & 'a ExecutableDocument , factory : F) where V : Visitor < 'a > + 'a , F : Fn () -> V , { if let Err (errors) = validate (doc , factory) { for err in errors { if let Some (position) = err . locations . first () { print ! ("[{}:{}] " , position . line , position . column) ; } println ! ("{}" , err . message) ; } panic ! ("Expected rule to pass, but errors found") ; } }
};
}
