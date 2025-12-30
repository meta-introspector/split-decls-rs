// Generated macro for impl_955 (impl)
macro_rules! Depcrate_query_builder_insert_statement_insert_from_selectimpl_955 {
() => {
// Module: crate::query_builder::insert_statement::insert_from_select
// Provides: {"impl_955"}
// Dependencies: {}
impl < DB , Select , Columns > QueryFragment < DB > for InsertFromSelect < Select , Columns > where DB : Backend , Columns : ColumnList + Expression , Select : Query < SqlType = Columns :: SqlType > + QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("(") ; self . columns . walk_ast (out . reborrow ()) ? ; out . push_sql (") ") ; self . query . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
