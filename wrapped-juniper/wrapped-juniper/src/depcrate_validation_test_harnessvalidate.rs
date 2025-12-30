// Generated macro for validate (function)
macro_rules! Depcrate_validation_test_harnessvalidate {
() => {
// Module: crate::validation::test_harness
// Provides: {"validate"}
// Dependencies: {}
pub (crate) fn validate < 'a , Q , M , Sub , F , S > (r : Q , m : M , s : Sub , q : & 'a str , visit_fn : F ,) -> Vec < RuleError > where S : ScalarValue + 'a , Q : GraphQLType < S , TypeInfo = () > , M : GraphQLType < S , TypeInfo = () > , Sub : GraphQLType < S , TypeInfo = () > , F : FnOnce (& mut ValidatorContext < 'a , S > , & 'a Document < S >) , { let mut root = RootNode :: new_with_scalar_value (r , m , s) ; root . schema . add_directive (DirectiveType :: new ("onQuery" , & [DirectiveLocation :: Query] , & [] , false ,)) ; root . schema . add_directive (DirectiveType :: new ("onMutation" , & [DirectiveLocation :: Mutation] , & [] , false ,)) ; root . schema . add_directive (DirectiveType :: new ("onField" , & [DirectiveLocation :: Field] , & [] , false ,)) ; root . schema . add_directive (DirectiveType :: new ("onFragmentDefinition" , & [DirectiveLocation :: FragmentDefinition] , & [] , false ,)) ; root . schema . add_directive (DirectiveType :: new ("onFragmentSpread" , & [DirectiveLocation :: FragmentSpread] , & [] , false ,)) ; root . schema . add_directive (DirectiveType :: new ("onInlineFragment" , & [DirectiveLocation :: InlineFragment] , & [] , false ,)) ; let doc = parse_document_source (q , & root . schema) . unwrap_or_else (| _ | panic ! ("Parse error on input {q:#?}")) ; let mut ctx = ValidatorContext :: new (unsafe { mem :: transmute (& root . schema) } , & doc) ; visit_fn (& mut ctx , unsafe { mem :: transmute (doc . as_slice ()) }) ; ctx . into_errors () }
};
}
