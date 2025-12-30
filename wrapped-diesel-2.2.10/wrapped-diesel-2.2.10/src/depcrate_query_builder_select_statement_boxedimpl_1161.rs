// Generated macro for impl_1161 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1161 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1161"}
// Dependencies: {}
impl < 'a , ST , QS , DB , GB > BoxedQueryHelper < 'a , QS , DB > for BoxedSelectStatement < 'a , ST , QS , DB , GB > { fn build_query < 'b , 'c > (& 'b self , mut out : AstPass < '_ , 'c , DB > , where_clause_handler : impl Fn (& 'b BoxedWhereClause < 'a , DB > , AstPass < '_ , 'c , DB > ,) -> QueryResult < () > ,) -> QueryResult < () > where DB : Backend , QS : QueryFragment < DB > , BoxedLimitOffsetClause < 'a , DB > : QueryFragment < DB > , 'b : 'c , { out . push_sql ("SELECT ") ; self . distinct . walk_ast (out . reborrow ()) ? ; self . select . walk_ast (out . reborrow ()) ? ; self . from . walk_ast (out . reborrow ()) ? ; where_clause_handler (& self . where_clause , out . reborrow ()) ? ; self . group_by . walk_ast (out . reborrow ()) ? ; self . having . walk_ast (out . reborrow ()) ? ; if let Some (ref order) = self . order { out . push_sql (" ORDER BY ") ; order . walk_ast (out . reborrow ()) ? ; } self . limit_offset . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
