// Generated macro for impl_255 (impl)
macro_rules! Depcrate_parse_rstestimpl_255 {
() => {
// Module: crate::parse::rstest
// Provides: {"impl_255"}
// Dependencies: {}
impl ToTokens for RsTestItem { fn to_tokens (& self , tokens : & mut TokenStream) { use RsTestItem :: * ; match self { Fixture (ref fixture) => fixture . to_tokens (tokens) , CaseArgName (ref case_arg) => case_arg . to_tokens (tokens) , TestCase (ref case) => case . to_tokens (tokens) , ValueList (ref list) => list . to_tokens (tokens) , } } }
};
}
