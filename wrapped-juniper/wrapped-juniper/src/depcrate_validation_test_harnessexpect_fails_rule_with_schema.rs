// Generated macro for expect_fails_rule_with_schema (function)
macro_rules! Depcrate_validation_test_harnessexpect_fails_rule_with_schema {
() => {
// Module: crate::validation::test_harness
// Provides: {"expect_fails_rule_with_schema"}
// Dependencies: {}
pub (crate) fn expect_fails_rule_with_schema < 'a , Q , M , V , F , S > (r : Q , m : M , factory : F , q : & 'a str , expected_errors : & [RuleError] ,) where S : ScalarValue + 'a , Q : GraphQLType < S , TypeInfo = () > , M : GraphQLType < S , TypeInfo = () > , V : Visitor < 'a , S > + 'a , F : FnOnce () -> V , { let errs = validate (r , m , crate :: EmptySubscription :: < S > :: new () , q , move | ctx , doc | { let mut mv = MultiVisitorNil . with (factory ()) ; visit (& mut mv , ctx , unsafe { mem :: transmute (doc) }) ; } ,) ; if errs . is_empty () { panic ! ("Expected rule to fail, but no errors were found") ; } else if errs != expected_errors { println ! ("==> Expected errors:") ; print_errors (expected_errors) ; println ! ("\n==> Actual errors:") ; print_errors (& errs) ; panic ! ("Unexpected set of errors found") ; } }
};
}
