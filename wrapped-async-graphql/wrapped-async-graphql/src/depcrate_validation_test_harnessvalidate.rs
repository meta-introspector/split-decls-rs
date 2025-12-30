// Generated macro for validate (function)
macro_rules! Depcrate_validation_test_harnessvalidate {
() => {
// Module: crate::validation::test_harness
// Provides: {"validate"}
// Dependencies: {}
pub (crate) fn validate < 'a , V , F > (doc : & 'a ExecutableDocument , factory : F ,) -> Result < () , Vec < RuleError > > where V : Visitor < 'a > + 'a , F : Fn () -> V , { let schema = TEST_HARNESS . get_or_init (| | Schema :: new (Query , Mutation , Subscription)) ; let registry = & schema . 0 . env . registry ; let mut ctx = VisitorContext :: new (registry , doc , None) ; let mut visitor = factory () ; visit (& mut visitor , & mut ctx , doc) ; if ctx . errors . is_empty () { Ok (()) } else { Err (ctx . errors) } }
};
}
