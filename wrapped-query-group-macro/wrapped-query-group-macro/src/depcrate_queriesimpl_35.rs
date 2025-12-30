// Generated macro for impl_35 (impl)
macro_rules! Depcrate_queriesimpl_35 {
() => {
// Module: crate::queries
// Provides: {"impl_35"}
// Dependencies: {}
impl ToTokens for Queries { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { match self { Queries :: TrackedQuery (tracked_query) => tracked_query . to_tokens (tokens) , Queries :: InputQuery (input_query) => input_query . to_tokens (tokens) , Queries :: Transparent (transparent) => transparent . to_tokens (tokens) , Queries :: Intern (intern) => intern . to_tokens (tokens) , } } }
};
}
