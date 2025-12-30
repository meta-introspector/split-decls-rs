// Generated macro for expect_fails_fn_with_schema (function)
macro_rules! Depcrate_validation_test_harnessexpect_fails_fn_with_schema {
() => {
// Module: crate::validation::test_harness
// Provides: {"expect_fails_fn_with_schema"}
// Dependencies: {}
pub (crate) fn expect_fails_fn_with_schema < 'a , Q , M , F , S > (r : Q , m : M , visit_fn : F , q : & 'a str , expected_errors : & [RuleError] ,) where S : ScalarValue + 'a , Q : GraphQLType < S , TypeInfo = () > , M : GraphQLType < S , TypeInfo = () > , F : FnOnce (& mut ValidatorContext < 'a , S > , & 'a Document < S >) , { let errs = validate (r , m , crate :: EmptySubscription :: < S > :: new () , q , visit_fn) ; if errs . is_empty () { panic ! ("Expected `visit_fn` to fail, but no errors were found") ; } else if errs != expected_errors { println ! ("==> Expected errors:") ; print_errors (expected_errors) ; println ! ("\n==> Actual errors:") ; print_errors (& errs) ; panic ! ("Unexpected set of errors found") ; } }
};
}
