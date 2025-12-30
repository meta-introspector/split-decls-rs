// Generated macro for impl_1051 (impl)
macro_rules! Depcrate_query_builder_insert_statementimpl_1051 {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"impl_1051"}
// Dependencies: {}
impl < T , Tab , DB > QueryFragment < DB > for ValuesClause < T , Tab > where DB : Backend , Tab : Table , T : InsertValues < DB , Tab > , DefaultValues : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { if self . values . is_noop (out . backend ()) ? { DefaultValues . walk_ast (out) ? ; } else { out . push_sql ("(") ; self . values . column_names (out . reborrow ()) ? ; out . push_sql (") VALUES (") ; self . values . walk_ast (out . reborrow ()) ? ; out . push_sql (")") ; } Ok (()) } }
};
}
