// Generated macro for impl_511 (impl)
macro_rules! Depcrate_expression_operatorsimpl_511 {
() => {
// Module: crate::expression::operators
// Provides: {"impl_511"}
// Dependencies: {}
impl < L , R , DB > QueryFragment < DB , sql_dialect :: concat_clause :: ConcatWithPipesClause > for Concat < L , R > where L : QueryFragment < DB > , R : QueryFragment < DB > , DB : Backend + SqlDialect < ConcatClause = sql_dialect :: concat_clause :: ConcatWithPipesClause > , { fn walk_ast < 'b > (& 'b self , mut out : crate :: query_builder :: AstPass < '_ , 'b , DB > ,) -> crate :: result :: QueryResult < () > { out . push_sql ("(") ; self . left . walk_ast (out . reborrow ()) ? ; out . push_sql (" || ") ; self . right . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; Ok (()) } }
};
}
