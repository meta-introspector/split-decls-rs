// Generated macro for get_recursion_limit (function)
macro_rules! Depcrate_passesget_recursion_limit {
() => {
// Module: crate::passes
// Provides: {"get_recursion_limit"}
// Dependencies: {}
fn get_recursion_limit (krate_attrs : & [ast :: Attribute] , sess : & Session) -> Limit { let attr = AttributeParser :: parse_limited_should_emit (sess , & krate_attrs , sym :: recursion_limit , DUMMY_SP , rustc_ast :: node_id :: CRATE_NODE_ID , None , ShouldEmit :: EarlyFatal { also_emit_lints : false } ,) ; crate :: limits :: get_recursion_limit (attr . as_slice ()) }
};
}
