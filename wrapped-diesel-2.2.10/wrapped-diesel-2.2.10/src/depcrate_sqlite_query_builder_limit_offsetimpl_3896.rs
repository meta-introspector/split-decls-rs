// Generated macro for impl_3896 (impl)
macro_rules! Depcrate_sqlite_query_builder_limit_offsetimpl_3896 {
() => {
// Module: crate::sqlite::query_builder::limit_offset
// Provides: {"impl_3896"}
// Dependencies: {}
impl QueryFragment < Sqlite > for BoxedLimitOffsetClause < '_ , Sqlite > { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , Sqlite >) -> QueryResult < () > { match (self . limit . as_ref () , self . offset . as_ref ()) { (Some (limit) , Some (offset)) => { limit . walk_ast (out . reborrow ()) ? ; offset . walk_ast (out . reborrow ()) ? ; } (Some (limit) , None) => { limit . walk_ast (out . reborrow ()) ? ; } (None , Some (offset)) => { out . push_sql (" LIMIT -1 ") ; offset . walk_ast (out . reborrow ()) ? ; } (None , None) => { } } Ok (()) } }
};
}
