// Generated macro for expect_passes_rule_with_schema (function)
macro_rules! Depcrate_validation_test_harnessexpect_passes_rule_with_schema {
() => {
// Module: crate::validation::test_harness
// Provides: {"expect_passes_rule_with_schema"}
// Dependencies: {}
pub (crate) fn expect_passes_rule_with_schema < 'a , Q , M , Sub , V , F , S > (r : Q , m : M , s : Sub , factory : F , q : & 'a str ,) where S : ScalarValue + 'a , Q : GraphQLType < S , TypeInfo = () > , M : GraphQLType < S , TypeInfo = () > , Sub : GraphQLType < S , TypeInfo = () > , V : Visitor < 'a , S > + 'a , F : FnOnce () -> V , { let errs = validate (r , m , s , q , move | ctx , doc | { let mut mv = MultiVisitorNil . with (factory ()) ; visit (& mut mv , ctx , unsafe { mem :: transmute (doc) }) ; }) ; if ! errs . is_empty () { print_errors (& errs) ; panic ! ("Expected rule to pass, but errors found") ; } }
};
}
