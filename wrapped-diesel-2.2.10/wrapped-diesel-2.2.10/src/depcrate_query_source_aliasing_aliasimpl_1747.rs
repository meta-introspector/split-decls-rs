// Generated macro for impl_1747 (impl)
macro_rules! Depcrate_query_source_aliasing_aliasimpl_1747 {
() => {
// Module: crate::query_source::aliasing::alias
// Provides: {"impl_1747"}
// Dependencies: {}
impl < S , DB > QueryFragment < DB , sql_dialect :: alias_syntax :: AsAliasSyntax > for Alias < S > where S : AliasSource , DB : Backend , S :: Target : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . source . target () . walk_ast (pass . reborrow ()) ? ; pass . push_sql (" AS ") ; pass . push_identifier (S :: NAME) ? ; Ok (()) } }
};
}
